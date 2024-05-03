//  warn!("my_var == {:?}", my_var);

use log::{LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Handle,
};
use std::fs::File;
use std::io::Write;

pub fn log_initialize(file_path: &str) -> Result<Handle, SetLoggerError> {
    let mut clear_log = File::create(file_path).expect("log-create-err");
    clear_log.write_all(b"").expect("log-clear-err");

    let log_level = log::LevelFilter::Error;
    let std_err = ConsoleAppender::builder().target(Target::Stderr).build();

    let log_file = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)
        .expect("log-append-err");

    let log_config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(log_file)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(log_level)))
                .build("stderr", Box::new(std_err)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                //.build(LevelFilter::Warn),
                .build(LevelFilter::Trace),
        )
        .expect("log-config-err");
    let _handle = log4rs::init_config(log_config);
    _handle
}

pub fn log_trace_off(level_filter: LevelFilter) {
    log::set_max_level(level_filter);
}
