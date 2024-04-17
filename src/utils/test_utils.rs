// use log::LevelFilter;
// use log4rs::append::console::ConsoleAppender;
// use log4rs::append::file::FileAppender;
// use log4rs::config::{Appender, Config, Logger, Root};
// use log4rs::encode::pattern::PatternEncoder;
use std::sync::Once;
static INIT: Once = Once::new();

pub fn setup_test_logger() {
    #[cfg(debug_assertions)]
    {
        // INIT.call_once(|| {
        //     let stdout = ConsoleAppender::builder().build();
        //
        //     let config = Config::builder()
        //         .appender(Appender::builder().build("stdout", Box::new(stdout)))
        //         //.appender(Appender::builder().build("requests", Box::new(requests)))
        //         .logger(Logger::builder().build("voidforge", LevelFilter::Info))
        //         // .logger(Logger::builder()
        //         //     .appender("requests")
        //         //     .additive(false)
        //         //     .build("app::requests", LevelFilter::Info))
        //         .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        //         .unwrap();
        //
        //     let handle = log4rs::init_config(config).unwrap();
        //
        //     // println!("Logger initialized.")
        // });
        INIT.call_once(|| {
            log4rs::init_file("log_config.yml", Default::default()).unwrap();
        });
    }
}

pub const _CONSOLE_RED: &str = "\x1b[31m";
pub const _CONSOLE_GREEN: &str = "\x1b[32m";
pub const _CONSOLE_BRIGHT_GREEN: &str = "\x1b[92m";
pub const _CONSOLE_YELLOW: &str = "\x1b[33m";
pub const _CONSOLE_BLUE: &str = "\x1b[34m";
pub const _CONSOLE_BRIGHT_BLUE: &str = "\x1b[94m";
pub const _CONSOLE_DEFAULT_COLOR: &str = "\x1b[0m";
