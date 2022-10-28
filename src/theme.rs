use cursive::{theme::BorderStyle, Cursive};

pub fn setup_theme(siv: &mut Cursive)
{
    siv.with_theme(|theme| {
        use cursive::theme::{BaseColor::*, Color::*, PaletteColor::*};

        theme.shadow = false;
        theme.palette[Background] = TerminalDefault;
        theme.palette[Primary] = TerminalDefault;
        theme.palette[View] = TerminalDefault;
        theme.palette[Highlight] = Magenta.dark();
        theme.palette[HighlightInactive] = Magenta.dark();
        theme.palette[HighlightText] = Black.dark();
        theme.palette[Secondary] = TerminalDefault;
        theme.palette[Tertiary] = TerminalDefault;
        theme.palette[TitlePrimary] = TerminalDefault;
        theme.palette[TitleSecondary] = TerminalDefault;

        theme.borders = BorderStyle::Simple;
    });
}
