use actix_web::{web, App, HttpServer};

use auth_service::cli_parser::CLIParsedArgs;
use auth_service::email_adapter::EmailAdapter;
use auth_service::mongo_adapter::StorageAdapter;
use auth_service::service_state::{ServiceAppConfig, ServiceAppState};

use std::sync::Mutex;

pub mod logger;
pub mod redis_adapter;
pub mod register_routing;
pub mod mongo_adapter;
pub mod service_state;
//pub mod cli_parser;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = CLIParsedArgs::new();
    let config = ServiceAppConfig::new(String::from("auth_service"));
    let email_adapter = EmailAdapter::new(config.get_email_credentials());
    //let adapter = redis_adapter::create_redis_adapter(cli.get_storage_address()).unwrap();
    let adapter = 
        match StorageAdapter::new(
            cli.get_storage_address(),
            config.get_app_name()).await {
                Ok(res) => res,
                Err(err) => {
                    panic!("{:?}", err);
                }
        };
    
    match cli.get_logger_file() {
        Some(path) => {
            logger::init_logger_by_file(path);
        },
        None => {
            logger::init_default_logger();
        }
    };

    let state = ServiceAppState {
        storage_adapter: Mutex::new(adapter),
        config,
    };

    let service_state = web::Data::new(state);

    HttpServer::new(move || {
        App::new()
            .app_data(service_state.clone())
            .service(register_routing::get_register_scope())
    })
    .bind(("127.0.0.1", cli.get_service_port()))?
    .run()
    .await
}
