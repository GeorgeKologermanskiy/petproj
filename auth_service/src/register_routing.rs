use actix_web::{web, Responder, HttpResponse, http::header::ContentType};
use serde::{Serialize, Deserialize};
use sha256::digest;
//use auth_service::mongo_adapter;
//use auth_service::service_state;

use crate::mongo_adapter::{UserDocument, InsertUserError};
use crate::service_state::ServiceAppState;

#[derive(Deserialize)]
struct RegisterInputInfo {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    code: u16,
    msg: String,
}

async fn register_account(state: web::Data<ServiceAppState>, info: web::Json<RegisterInputInfo>) -> impl Responder {
    // TODO: add params validation
    let user = UserDocument {
        user_id: digest(&info.username),
        username: info.username.clone(),
        email: Some(info.email.clone()),
        email_confirmed: false,
        password: info.password.clone()
    };
    let mut storage_adapter = state.storage_adapter.lock().unwrap();

    // Insert user in storage
    let insert_result = storage_adapter.insert_user(&user).await;
    if insert_result.is_err() {
        let bad_resp = match insert_result.err().unwrap() {
            InsertUserError::FoundUserWithSameID => {
                RegisterResponse {
                    code: 1,
                    msg: String::from(""),
                }
            },
            InsertUserError::InternalError => {
                RegisterResponse {
                    code: 500,
                    msg: String::from("Internal service error"),
                }
            }
        };
        return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&bad_resp).unwrap());
    }

    // Generate confirmation
    let confirm_result = storage_adapter
            .insert_register_confirmation(user.user_id.clone()).await;
    if confirm_result.is_err() {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&RegisterResponse {
                code: 500,
                msg: String::from("Internal service error"),
            }).unwrap());
    }

    // TODO: Generate link, send it to email
    let link = format!("http://127.0.0.1/api/v1/confirm?token={}",
        confirm_result.ok().unwrap());
    println!("Generated link: {}", link);

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&RegisterResponse {
            code: 0,
            msg: String::from("Registered account, check email"),
        }).unwrap())
}

#[derive(Deserialize)]
struct ConfirmRegistrationInfo {
    token: String
}

async fn confirm_registration(
    _state: web::Data<ServiceAppState>, _info: web::Query<ConfirmRegistrationInfo>) -> impl Responder {
    "Confirmed"
}

pub fn get_register_scope() -> actix_web::Scope {
    web::scope("/api/v1")
        .route("/register", web::post().to(register_account))
        .route("/confirm", web::get().to(confirm_registration))
}