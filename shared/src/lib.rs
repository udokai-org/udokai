use log::{LevelFilter, SetLoggerError};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

pub fn setup_logger(file: &str) -> Result<log4rs::Handle, SetLoggerError> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file).expect("Failed to create logfile");

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Info)).expect("Failed to create config");

    log4rs::init_config(config)
}
