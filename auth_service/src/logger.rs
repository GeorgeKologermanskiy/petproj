use log4rs::init_file;
use std::path::PathBuf;

pub fn init_logger_by_file(log_filename: &PathBuf) {
    init_file(log_filename.as_path(), Default::default()).unwrap()
}

pub fn init_default_logger() {
    // let config = Config::builder()
    //    .appender()
    //    .build()
    //    .unwrap();

    // init_config(config).unwrap()
}
