mod common;

use actix_web::{test, http::StatusCode};
use serde_json::json;
use serial_test::serial;

#[actix_rt::test]
#[serial]
async fn test_utilizadores_get_all() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    let req = test::TestRequest::get()
        .uri("/api/utilizadores")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
#[serial]
async fn test_utilizadores_create_and_get() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    // Create utilizador
    let create_payload = json!({
        "nome": "João Silva",
        "email": "joao.silva@example.com"
    });

    let req = test::TestRequest::post()
        .uri("/api/utilizadores")
        .set_json(&create_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let utilizador_id = body["id"].as_str().unwrap();

    // Get utilizador by ID
    let req = test::TestRequest::get()
        .uri(&format!("/api/utilizadores/{}", utilizador_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["nome"].as_str().unwrap(), "João Silva");
    assert_eq!(body["email"].as_str().unwrap(), "joao.silva@example.com");
}

#[actix_rt::test]
#[serial]
async fn test_utilizadores_create_invalid_email() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    let create_payload = json!({
        "nome": "Teste",
        "email": "email-invalido"
    });

    let req = test::TestRequest::post()
        .uri("/api/utilizadores")
        .set_json(&create_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
#[serial]
async fn test_utilizadores_create_nome_vazio() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    let create_payload = json!({
        "nome": "",
        "email": "test@example.com"
    });

    let req = test::TestRequest::post()
        .uri("/api/utilizadores")
        .set_json(&create_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
#[serial]
async fn test_utilizadores_update() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    // Create utilizador
    let create_payload = json!({
        "nome": "Maria Santos",
        "email": "maria@example.com"
    });

    let req = test::TestRequest::post()
        .uri("/api/utilizadores")
        .set_json(&create_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let utilizador_id = body["id"].as_str().unwrap();

    // Update utilizador
    let update_payload = json!({
        "nome": "Maria Silva",
        "email": "maria.silva@example.com"
    });

    let req = test::TestRequest::put()
        .uri(&format!("/api/utilizadores/{}", utilizador_id))
        .set_json(&update_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["nome"].as_str().unwrap(), "Maria Silva");
    assert_eq!(body["email"].as_str().unwrap(), "maria.silva@example.com");
}

#[actix_rt::test]
#[serial]
async fn test_utilizadores_delete() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    // Create utilizador
    let create_payload = json!({
        "nome": "Pedro Oliveira",
        "email": "pedro@example.com"
    });

    let req = test::TestRequest::post()
        .uri("/api/utilizadores")
        .set_json(&create_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let utilizador_id = body["id"].as_str().unwrap();

    // Delete
    let req = test::TestRequest::delete()
        .uri(&format!("/api/utilizadores/{}", utilizador_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Verify deleted
    let req = test::TestRequest::get()
        .uri(&format!("/api/utilizadores/{}", utilizador_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
#[serial]
async fn test_utilizadores_get_nonexistent() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    let fake_id = "00000000-0000-0000-0000-000000000000";
    
    let req = test::TestRequest::get()
        .uri(&format!("/api/utilizadores/{}", fake_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
