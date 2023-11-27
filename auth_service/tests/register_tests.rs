use actix_web::{test, App};
use auth_service;

#[actix_web::test]
async fn simple_register_test() {
    let app = test::init_service(
        App::new()
            .service(auth_service::register_routing::get_register_scope())
    ).await;

    // try register
    let req = test::TestRequest::default()
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    println!("HERE");
}
