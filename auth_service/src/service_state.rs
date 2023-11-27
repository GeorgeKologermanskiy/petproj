use crate::mongo_adapter;
use std::env;
use std::sync::Mutex;

// host, username and password
pub struct EmailAdapterCredentials(pub String, pub String, pub String);

pub struct ServiceAppConfig {
    app_name: String,
    email_adapter_credentials: EmailAdapterCredentials,
}

impl ServiceAppConfig {

    pub fn new(app_name: String) -> Self {
        let email_client_host = Self::get_var("EMAIL_CLIENT_HOST");
        let email_client_username = Self::get_var("EMAIL_CLIENT_USERNAME");
        let email_client_password = Self::get_var("EMAIL_CLIENT_PASSWORD");

        let credentials = EmailAdapterCredentials(
            email_client_host,
            email_client_username,
            email_client_password,
        );

        Self {
            app_name,
            email_adapter_credentials : credentials,
        }
    }

    fn get_var(key: &str) -> String {
        match env::var(key) {
            Ok(val) => val,
            Err(e) => {
                panic!("{key} not found {e:?}");
            }
        }
    }

    pub fn get_app_name(&self) -> &String {
        &self.app_name
    }

    pub fn get_email_credentials(&self) -> &EmailAdapterCredentials {
        &self.email_adapter_credentials
    }

}

pub struct ServiceAppState {
    pub storage_adapter: Mutex<mongo_adapter::StorageAdapter>,
    pub config: ServiceAppConfig,
}

//impl ServiceAppState {
    //pub fn get_adapter(&mut self) -> &mut mongo_adapter::StorageAdapter {
    //    &mut self.storage_adapter
    //}
//}
