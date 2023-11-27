use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

pub mod cli_parser;
use cli_parser::CLIParsedArgs;

pub mod email_adapter;
use email_adapter::EmailAdapter;

pub mod logger;

pub mod mongo_adapter;
use mongo_adapter::StorageAdapter;

pub mod redis_adapter;
pub mod register_routing;

pub mod service_state;
use service_state::{ServiceAppConfig, ServiceAppState};

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = CLIParsedArgs::new();
    let config = ServiceAppConfig::new(String::from("auth_service"));
    let _email_adapter = EmailAdapter::new(config.get_email_credentials());
    //let adapter = redis_adapter::create_redis_adapter(cli.get_storage_address()).unwrap();
    let adapter = match StorageAdapter::new(cli.get_storage_address(), config.get_app_name()).await
    {
        Ok(res) => res,
        Err(err) => {
            panic!("{:?}", err);
        }
    };

    match cli.get_logger_file() {
        Some(path) => {
            logger::init_logger_by_file(path);
        }
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
