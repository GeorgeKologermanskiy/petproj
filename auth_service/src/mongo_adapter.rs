use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct StorageAdapter {
    db_name: String,
    client: mongodb::Client,
}

pub enum InsertUserError {
    FoundUserWithSameID,
    InternalError,
}

#[derive(Serialize, Deserialize)]
pub struct UserDocument {
    pub user_id: String,
    pub username: String,
    pub email_confirmed: bool,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct ConfirmDocument {
    pub token: String,
    pub user_id: String,
}

impl StorageAdapter {
    pub async fn new(address: &String, db_name: &String) -> Result<Self, mongodb::error::Error> {
        let mut client_options = ClientOptions::parse(address).await?;

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options)?;

        Ok(Self {
            db_name: db_name.clone(),
            client,
        })
    }

    pub async fn insert_user(&mut self, user: &UserDocument) -> Result<(), InsertUserError> {
        let collection: mongodb::Collection<UserDocument> =
            self.client.database(&self.db_name).collection("users");
        let result = collection.insert_one(user, None).await;
        match result {
            Ok(res) => {
                println!("{:?}", res);
                Ok(())
            }
            Err(e) => {
                println!("{:?}", e);
                Err(InsertUserError::InternalError)
            }
        }
    }

    pub async fn insert_register_confirmation(&mut self, user_id: String) -> Result<String, ()> {
        let collection: mongodb::Collection<ConfirmDocument> = self
            .client
            .database(&self.db_name)
            .collection("confirmations");
        let token = Uuid::new_v4().to_string();
        let confirmation = ConfirmDocument {
            token: token.clone(),
            user_id,
        };
        let result = collection.insert_one(confirmation, None).await;
        match result {
            Ok(res) => {
                println!("{:?}", res);
                Ok(token)
            }
            Err(e) => {
                println!("{:?}", e);
                Err(())
            }
        }
    }
}
