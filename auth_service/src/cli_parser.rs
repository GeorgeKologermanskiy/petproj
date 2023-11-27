use std::fs;
use std::path::PathBuf;

use clap::Parser;

// default service attach port
const DEFAULT_SERVICE_PORT: u16 = 5000;
// default storage port
const DEFAULT_STORAGE_ADDRESS: &str = "mongodb://127.0.0.1:27017";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct CLIArgs {
    #[arg(long, default_value_t = DEFAULT_SERVICE_PORT)]
    service_port: u16,

    #[arg(long, default_value_t = String::from(DEFAULT_STORAGE_ADDRESS))]
    storage_address: String,

    #[arg(long)]
    logger_file: Option<PathBuf>,
}

pub struct CLIParsedArgs {
    service_port: u16,
    storage_address: String,
    logger_file: Option<PathBuf>,
}

impl CLIParsedArgs {
    pub fn new() -> Self {
        let args = CLIArgs::parse();
        if args.logger_file.is_some() {
            // check path exists & it is readable file
            match fs::metadata(args.logger_file.as_ref().unwrap()) {
                Ok(md) => {
                    if !md.is_file() {
                        panic!("Not a file");
                    }
                }
                Err(err) => {
                    panic!("{err:}");
                }
            }
        }
        Self {
            service_port: args.service_port,
            storage_address: args.storage_address,
            logger_file: args.logger_file,
        }
    }

    pub fn get_service_port(&self) -> u16 {
        self.service_port
    }

    pub fn get_storage_address(&self) -> &String {
        &self.storage_address
    }

    pub fn get_logger_file(&self) -> &Option<PathBuf> {
        &self.logger_file
    }
}
