// tests/health_controller_test.rs
// Equivalent de: tests/Controller/HealthControllerTest.php

mod common;

use axum::http::StatusCode;

/// Test GET / returns 200 and hello message
/// Equivalent de: testIndex() en Symfony
#[tokio::test]
async fn test_index_returns_hello() {
    // Arrange - Create test client (like static::createClient())
    let server = common::create_test_server().await;

    // Act - Send GET request (like $client->request('GET', '/'))
    let response = server.get("/").await;

    // Assert - Check response (like $this->assertResponseIsSuccessful())
    response.assert_status(StatusCode::OK);
    response.assert_text("Hello from Rust API!");
}

/// Test GET /health returns 200 and OK
/// Equivalent de: testHealth() en Symfony
#[tokio::test]
async fn test_health_returns_ok() {
    let server = common::create_test_server().await;

    let response = server.get("/health").await;

    response.assert_status(StatusCode::OK);
    response.assert_text("OK");
}
