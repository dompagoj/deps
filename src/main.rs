use cursive::{
    align::HAlign,
    direction::{Absolute, Direction, Orientation},
    event::{self, Event, EventResult},
    traits::*,
    view::{Margins, Scrollable, Selector},
    views::{Dialog, DummyView, EditView, LinearLayout, ListView, OnEventView, PaddedView, SelectView, TextView},
    Cursive,
};

use logging::debug_d;
use providers::NpmProvider;

mod data;
mod json;
mod logging;
mod providers;
mod theme;
mod ui;
mod version;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    logging::init();
    let mut siv = cursive::default();
    theme::setup_theme(&mut siv);

    siv.add_layer(Dialog::around(search_view()).title("Enter package name"));

    siv.try_run()
}

fn search_view() -> impl View
{
    let on_edit = |siv: &mut Cursive, query: &str, _cursor: usize| {
        let cb_sink = siv.cb_sink().clone();

        let query_clone = query.to_lowercase().trim_end().trim_start().to_string();

        let future = async move {
            let matches = match query_clone.is_empty() {
                true => vec![],
                false => NpmProvider::search_npm_packages_by_term(&query_clone).await,
            };

            cb_sink
                .send(Box::new(move |s| {
                    s.call_on_name("select", |v: &mut SelectView| {
                        v.clear();
                        v.add_all_str(
                            matches
                                .into_iter()
                                .map(|package| format!("{} --- @{}", package.name, package.version.to_string()))
                                .collect::<Vec<String>>(),
                        );
                    });
                }))
                .unwrap();
        };
        tokio::spawn(future);
    };

    LinearLayout::vertical()
        .child(
            EditView::new()
                .on_edit(on_edit)
                .on_submit(|_s, _package_name| {
                    // TODO: Run npm install with package name
                    debug_d!("Run npm install");
                })
                .with_name("edit"),
        )
        .child(PaddedView::new(Margins::lrtb(0, 0, 2, 1), TextView::new("Packages: ")))
        .child(
            SelectView::new()
                .h_align(HAlign::Left)
                .with_all_str(["Type to search"])
                .on_select(|s, item| {
                    s.call_on_name("edit", |edit: &mut EditView| {
                        let split = item.split("---").collect::<Vec<&str>>();
                        edit.set_content(split[0].trim_end());
                    });
                })
                .autojump()
                .with_name("select")
                .scrollable(),
        )
        .min_height(15)
        .min_width(25)
}
