pub use log;

macro_rules! debug_d {
    ($s: literal) => {
        log::debug!($s)
    };
    ($i:ident) => {};
    ($s:literal, $i: ident) => {
        log::debug!("{} : {:?}", $s, $i)
    };
}

pub fn init()
{
    env_logger::builder()
        .format_timestamp(None)
        .format_level(false)
        .format_target(false)
        .init();
}

pub(crate) use debug_d;
