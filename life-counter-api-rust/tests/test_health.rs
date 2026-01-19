mod common;

use actix_web::{test, http::StatusCode};
use serial_test::serial;

#[actix_rt::test]
#[serial]
async fn test_health_check_basic() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    let req = test::TestRequest::get()
        .uri("/health")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"].as_str().unwrap(), "healthy");
    assert!(body["timestamp"].is_string());
    assert!(body["version"].is_string());
}

#[actix_rt::test]
#[serial]
async fn test_health_check_detailed() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    let req = test::TestRequest::get()
        .uri("/health/detailed")
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    // Should be OK if database is available, or 503 if not
    assert!(
        resp.status() == StatusCode::OK || 
        resp.status() == StatusCode::SERVICE_UNAVAILABLE
    );

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["status"].is_string());
    assert!(body["timestamp"].is_string());
    assert!(body["version"].is_string());
    assert!(body["database"].is_object());
    assert!(body["database"]["status"].is_string());
    assert!(body["database"]["connected"].is_boolean());
}

#[actix_rt::test]
#[serial]
async fn test_health_check_response_structure() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    let req = test::TestRequest::get()
        .uri("/health")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;

    // Verify all required fields are present
    assert!(body.get("status").is_some());
    assert!(body.get("timestamp").is_some());
    assert!(body.get("version").is_some());
}

#[actix_rt::test]
#[serial]
async fn test_health_check_detailed_database_info() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    let req = test::TestRequest::get()
        .uri("/health/detailed")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;

    // Verify database health info is present
    let db = &body["database"];
    assert!(db.get("status").is_some());
    assert!(db.get("connected").is_some());
}

#[actix_rt::test]
#[serial]
async fn test_health_logs_endpoint() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    let req = test::TestRequest::get()
        .uri("/health/logs")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["total"].is_number());
    assert!(body["logs"].is_array());
}

#[actix_rt::test]
#[serial]
async fn test_health_logs_with_limit() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    let req = test::TestRequest::get()
        .uri("/health/logs?limit=10")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let total = body["total"].as_u64().unwrap();
    assert!(total <= 10);
}

#[actix_rt::test]
#[serial]
async fn test_health_logs_structure() {
    let pool = common::create_test_pool();
    let app = common::create_test_app(pool).await;

    let req = test::TestRequest::get()
        .uri("/health/logs?limit=1")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;

    // Verify response structure
    assert!(body.get("total").is_some());
    assert!(body.get("logs").is_some());
    
    let logs = body["logs"].as_array().unwrap();
    if !logs.is_empty() {
        let log = &logs[0];
        assert!(log.get("timestamp").is_some());
        assert!(log.get("level").is_some());
        assert!(log.get("message").is_some());
    }
}
