use actix_web::{http::header::ContentType, test, web, App};

#[actix_web::test]
async fn simple_test() {
    let app =
        test::init_service(App::new().service(web::resource("/test").to(|| async { "OK" }))).await;
    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
