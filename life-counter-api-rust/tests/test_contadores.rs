mod common;

use actix_web::{test, http::StatusCode};
use serde_json::json;
use serial_test::serial;

#[actix_rt::test]
#[serial]
async fn test_contadores_get_all() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    let req = test::TestRequest::get()
        .uri("/api/contadores")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
#[serial]
async fn test_contadores_create_and_get() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    // Create contador
    let create_payload = json!({
        "nome": "Contador de Teste",
        "valor": 10
    });

    let req = test::TestRequest::post()
        .uri("/api/contadores")
        .set_json(&create_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let contador_id = body["id"].as_str().unwrap();

    // Get contador by ID
    let req = test::TestRequest::get()
        .uri(&format!("/api/contadores/{}", contador_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["nome"].as_str().unwrap(), "Contador de Teste");
    assert_eq!(body["valor"].as_i64().unwrap(), 10);
}

#[actix_rt::test]
#[serial]
async fn test_contadores_create_invalid_nome() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    // Nome vazio
    let create_payload = json!({
        "nome": "",
        "valor": 0
    });

    let req = test::TestRequest::post()
        .uri("/api/contadores")
        .set_json(&create_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
#[serial]
async fn test_contadores_create_nome_muito_longo() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    let create_payload = json!({
        "nome": "a".repeat(201),
        "valor": 0
    });

    let req = test::TestRequest::post()
        .uri("/api/contadores")
        .set_json(&create_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
#[serial]
async fn test_contadores_update() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    // Create contador
    let create_payload = json!({
        "nome": "Contador Original",
        "valor": 5
    });

    let req = test::TestRequest::post()
        .uri("/api/contadores")
        .set_json(&create_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let contador_id = body["id"].as_str().unwrap();

    // Update contador
    let update_payload = json!({
        "nome": "Contador Atualizado",
        "valor": 15
    });

    let req = test::TestRequest::put()
        .uri(&format!("/api/contadores/{}", contador_id))
        .set_json(&update_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["nome"].as_str().unwrap(), "Contador Atualizado");
    assert_eq!(body["valor"].as_i64().unwrap(), 15);
}

#[actix_rt::test]
#[serial]
async fn test_contadores_incrementar() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    // Create contador
    let create_payload = json!({
        "nome": "Contador para Incrementar",
        "valor": 10
    });

    let req = test::TestRequest::post()
        .uri("/api/contadores")
        .set_json(&create_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let contador_id = body["id"].as_str().unwrap();

    // Incrementar
    let increment_payload = json!({
        "incremento": 5
    });

    let req = test::TestRequest::patch()
        .uri(&format!("/api/contadores/{}/incrementar", contador_id))
        .set_json(&increment_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["valor"].as_i64().unwrap(), 15);
}

#[actix_rt::test]
#[serial]
async fn test_contadores_delete() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    // Create contador
    let create_payload = json!({
        "nome": "Contador para Deletar",
        "valor": 0
    });

    let req = test::TestRequest::post()
        .uri("/api/contadores")
        .set_json(&create_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let contador_id = body["id"].as_str().unwrap();

    // Delete
    let req = test::TestRequest::delete()
        .uri(&format!("/api/contadores/{}", contador_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Verify deleted
    let req = test::TestRequest::get()
        .uri(&format!("/api/contadores/{}", contador_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
#[serial]
async fn test_contadores_get_nonexistent() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    let fake_id = "00000000-0000-0000-0000-000000000000";
    
    let req = test::TestRequest::get()
        .uri(&format!("/api/contadores/{}", fake_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
