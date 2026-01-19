mod common;

use actix_web::{test, http::StatusCode};
use serde_json::json;
use serial_test::serial;

#[actix_rt::test]
#[serial]
async fn test_contador_utilizadores_create_and_get() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    // Create contador
    let contador_payload = json!({
        "nome": "Contador Associação",
        "valor": 5
    });

    let req = test::TestRequest::post()
        .uri("/api/contadores")
        .set_json(&contador_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let contador_id = body["id"].as_str().unwrap();

    // Create utilizador
    let utilizador_payload = json!({
        "nome": "Usuário Teste",
        "email": "usuario@example.com"
    });

    let req = test::TestRequest::post()
        .uri("/api/utilizadores")
        .set_json(&utilizador_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let utilizador_id = body["id"].as_str().unwrap();

    // Create association
    let assoc_payload = json!({
        "contador_id": contador_id,
        "utilizador_id": utilizador_id,
        "observacoes": "Teste de associação"
    });

    let req = test::TestRequest::post()
        .uri("/api/contador_utilizadores")
        .set_json(&assoc_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let assoc_id = body["id"].as_str().unwrap();
    assert_eq!(body["observacoes"].as_str().unwrap(), "Teste de associação");

    // Get association by ID
    let req = test::TestRequest::get()
        .uri(&format!("/api/contador_utilizadores/{}", assoc_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
#[serial]
async fn test_contador_utilizadores_get_all() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    let req = test::TestRequest::get()
        .uri("/api/contador_utilizadores")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
#[serial]
async fn test_contador_utilizadores_update() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    // Create contador
    let contador_payload = json!({
        "nome": "Contador Update",
        "valor": 0
    });

    let req = test::TestRequest::post()
        .uri("/api/contadores")
        .set_json(&contador_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let contador_id = body["id"].as_str().unwrap();

    // Create utilizador
    let utilizador_payload = json!({
        "nome": "Usuário Update",
        "email": "update@example.com"
    });

    let req = test::TestRequest::post()
        .uri("/api/utilizadores")
        .set_json(&utilizador_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let utilizador_id = body["id"].as_str().unwrap();

    // Create association
    let assoc_payload = json!({
        "contador_id": contador_id,
        "utilizador_id": utilizador_id,
        "observacoes": "Observação original"
    });

    let req = test::TestRequest::post()
        .uri("/api/contador_utilizadores")
        .set_json(&assoc_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let assoc_id = body["id"].as_str().unwrap();

    // Update association
    let update_payload = json!({
        "observacoes": "Observação atualizada"
    });

    let req = test::TestRequest::put()
        .uri(&format!("/api/contador_utilizadores/{}", assoc_id))
        .set_json(&update_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["observacoes"].as_str().unwrap(), "Observação atualizada");
}

#[actix_rt::test]
#[serial]
async fn test_contador_utilizadores_delete() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    // Create contador
    let contador_payload = json!({
        "nome": "Contador Delete",
        "valor": 0
    });

    let req = test::TestRequest::post()
        .uri("/api/contadores")
        .set_json(&contador_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let contador_id = body["id"].as_str().unwrap();

    // Create utilizador
    let utilizador_payload = json!({
        "nome": "Usuário Delete",
        "email": "delete@example.com"
    });

    let req = test::TestRequest::post()
        .uri("/api/utilizadores")
        .set_json(&utilizador_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let utilizador_id = body["id"].as_str().unwrap();

    // Create association
    let assoc_payload = json!({
        "contador_id": contador_id,
        "utilizador_id": utilizador_id
    });

    let req = test::TestRequest::post()
        .uri("/api/contador_utilizadores")
        .set_json(&assoc_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let assoc_id = body["id"].as_str().unwrap();

    // Delete association
    let req = test::TestRequest::delete()
        .uri(&format!("/api/contador_utilizadores/{}", assoc_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Verify deleted
    let req = test::TestRequest::get()
        .uri(&format!("/api/contador_utilizadores/{}", assoc_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
#[serial]
async fn test_contador_utilizadores_observacoes_muito_longas() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    // Create contador
    let contador_payload = json!({
        "nome": "Contador Teste",
        "valor": 0
    });

    let req = test::TestRequest::post()
        .uri("/api/contadores")
        .set_json(&contador_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let contador_id = body["id"].as_str().unwrap();

    // Create utilizador
    let utilizador_payload = json!({
        "nome": "Usuário Teste",
        "email": "test@example.com"
    });

    let req = test::TestRequest::post()
        .uri("/api/utilizadores")
        .set_json(&utilizador_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let utilizador_id = body["id"].as_str().unwrap();

    // Try to create association with observacoes too long
    let assoc_payload = json!({
        "contador_id": contador_id,
        "utilizador_id": utilizador_id,
        "observacoes": "a".repeat(501)
    });

    let req = test::TestRequest::post()
        .uri("/api/contador_utilizadores")
        .set_json(&assoc_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
#[serial]
async fn test_get_utilizadores_de_contador() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    // Create contador
    let contador_payload = json!({
        "nome": "Contador Relacionamento",
        "valor": 0
    });

    let req = test::TestRequest::post()
        .uri("/api/contadores")
        .set_json(&contador_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let contador_id = body["id"].as_str().unwrap();

    // Create utilizador
    let utilizador_payload = json!({
        "nome": "Usuário Relacionado",
        "email": "relacionado@example.com"
    });

    let req = test::TestRequest::post()
        .uri("/api/utilizadores")
        .set_json(&utilizador_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let utilizador_id = body["id"].as_str().unwrap();

    // Create association
    let assoc_payload = json!({
        "contador_id": contador_id,
        "utilizador_id": utilizador_id
    });

    let req = test::TestRequest::post()
        .uri("/api/contador_utilizadores")
        .set_json(&assoc_payload)
        .to_request();

    test::call_service(&app, req).await;

    // Get utilizadores of contador
    let req = test::TestRequest::get()
        .uri(&format!("/api/contadores/{}/utilizadores", contador_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let users = body.as_array().unwrap();
    assert!(users.len() >= 1);
}

#[actix_rt::test]
#[serial]
async fn test_get_contadores_de_utilizador() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    // Create contador
    let contador_payload = json!({
        "nome": "Contador do User",
        "valor": 0
    });

    let req = test::TestRequest::post()
        .uri("/api/contadores")
        .set_json(&contador_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let contador_id = body["id"].as_str().unwrap();

    // Create utilizador
    let utilizador_payload = json!({
        "nome": "User com Contador",
        "email": "user.contador@example.com"
    });

    let req = test::TestRequest::post()
        .uri("/api/utilizadores")
        .set_json(&utilizador_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let utilizador_id = body["id"].as_str().unwrap();

    // Create association
    let assoc_payload = json!({
        "contador_id": contador_id,
        "utilizador_id": utilizador_id
    });

    let req = test::TestRequest::post()
        .uri("/api/contador_utilizadores")
        .set_json(&assoc_payload)
        .to_request();

    test::call_service(&app, req).await;

    // Get contadores of utilizador
    let req = test::TestRequest::get()
        .uri(&format!("/api/utilizadores/{}/contadores", utilizador_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let contadores = body.as_array().unwrap();
    assert!(contadores.len() >= 1);
}
