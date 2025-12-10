// tests/user_controller_test.rs
// Equivalent de: tests/Controller/UserControllerTest.php

mod common;

use axum::http::StatusCode;
use serde_json::{json, Value};

// ============================================================
// GET /users - List users
// ============================================================

/// Test GET /users returns empty array initially with pagination meta
#[tokio::test]
async fn test_list_users_returns_empty_array() {
    let server = common::create_test_server().await;

    let response = server.get("/users").await;

    response.assert_status(StatusCode::OK);
    let body: Value = response.json();

    // Check { "data": [], "meta": { ... } } format
    assert!(body["data"].is_array());
    assert!(body["data"].as_array().unwrap().is_empty());

    // Check pagination meta
    let meta = &body["meta"];
    assert_eq!(meta["total"], 0);
    assert_eq!(meta["page"], 1);
    assert_eq!(meta["per_page"], 10);
    assert_eq!(meta["total_pages"], 0);
}

/// Test GET /users returns users after creation with pagination meta
#[tokio::test]
async fn test_list_users_returns_created_users() {
    let server = common::create_test_server().await;

    // Create a user first
    server
        .post("/users")
        .json(&json!({
            "username": "testuser",
            "email": "test@example.com"
        }))
        .await;

    let response = server.get("/users").await;

    response.assert_status(StatusCode::OK);
    let body: Value = response.json();

    // Check { "data": [...], "meta": { ... } } format
    let data = body["data"].as_array().unwrap();
    assert_eq!(data.len(), 1);
    assert_eq!(data[0]["username"], "testuser");
    assert_eq!(data[0]["email"], "test@example.com");

    // Check pagination meta
    let meta = &body["meta"];
    assert_eq!(meta["total"], 1);
    assert_eq!(meta["page"], 1);
    assert_eq!(meta["per_page"], 10);
    assert_eq!(meta["total_pages"], 1);
}

/// Test GET /users with pagination query params
#[tokio::test]
async fn test_list_users_with_pagination() {
    let server = common::create_test_server().await;

    // Create 5 users
    for i in 1..=5 {
        server
            .post("/users")
            .json(&json!({
                "username": format!("user{}", i),
                "email": format!("user{}@example.com", i)
            }))
            .await;
    }

    // Get page 1 with 2 items per page
    let response = server.get("/users?page=1&per_page=2").await;

    response.assert_status(StatusCode::OK);
    let body: Value = response.json();

    let data = body["data"].as_array().unwrap();
    assert_eq!(data.len(), 2);

    // Check pagination meta
    let meta = &body["meta"];
    assert_eq!(meta["total"], 5);
    assert_eq!(meta["page"], 1);
    assert_eq!(meta["per_page"], 2);
    assert_eq!(meta["total_pages"], 3);

    // Get page 3 (should have only 1 item)
    let response = server.get("/users?page=3&per_page=2").await;
    let body: Value = response.json();

    let data = body["data"].as_array().unwrap();
    assert_eq!(data.len(), 1);
    assert_eq!(body["meta"]["page"], 3);
}

// ============================================================
// GET /users/:id - Get single user
// ============================================================

/// Test GET /users/:id returns 404 for non-existent user
#[tokio::test]
async fn test_get_user_not_found() {
    let server = common::create_test_server().await;

    let response = server.get("/users/999").await;

    response.assert_status(StatusCode::NOT_FOUND);
    let body: Value = response.json();
    assert_eq!(body["error"], "Resource not found");
}

/// Test GET /users/:id returns user when exists
#[tokio::test]
async fn test_get_user_success() {
    let server = common::create_test_server().await;

    // Create a user
    let create_response = server
        .post("/users")
        .json(&json!({
            "username": "johndoe",
            "email": "john@example.com"
        }))
        .await;

    let created: Value = create_response.json();
    let user_id = created["data"]["id"].as_i64().unwrap();

    // Get the user
    let response = server.get(&format!("/users/{}", user_id)).await;

    response.assert_status(StatusCode::OK);
    let body: Value = response.json();

    // Check { "data": { ... } } format
    assert_eq!(body["data"]["id"], user_id);
    assert_eq!(body["data"]["username"], "johndoe");
    assert_eq!(body["data"]["email"], "john@example.com");
}

// ============================================================
// POST /users - Create user
// ============================================================

/// Test POST /users creates user successfully
#[tokio::test]
async fn test_create_user_success() {
    let server = common::create_test_server().await;

    let response = server
        .post("/users")
        .json(&json!({
            "username": "newuser",
            "email": "new@example.com"
        }))
        .await;

    response.assert_status(StatusCode::CREATED);
    let body: Value = response.json();

    // Check { "data": { ... } } format
    assert!(body["data"]["id"].as_i64().is_some());
    assert_eq!(body["data"]["username"], "newuser");
    assert_eq!(body["data"]["email"], "new@example.com");
    assert!(body["data"]["created_at"].as_str().is_some());
    // No meta for single item
    assert!(body["meta"].is_null());
}

/// Test POST /users returns 422 for invalid username (too short)
#[tokio::test]
async fn test_create_user_validation_error_username_too_short() {
    let server = common::create_test_server().await;

    let response = server
        .post("/users")
        .json(&json!({
            "username": "ab",
            "email": "valid@example.com"
        }))
        .await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    let body: Value = response.json();
    assert_eq!(body["error"], "Validation failed");
}

/// Test POST /users returns 422 for invalid email
#[tokio::test]
async fn test_create_user_validation_error_invalid_email() {
    let server = common::create_test_server().await;

    let response = server
        .post("/users")
        .json(&json!({
            "username": "validuser",
            "email": "not-an-email"
        }))
        .await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    let body: Value = response.json();
    assert_eq!(body["error"], "Validation failed");
}

/// Test POST /users returns 409 for duplicate email
#[tokio::test]
async fn test_create_user_duplicate_email() {
    let server = common::create_test_server().await;

    // Create first user
    server
        .post("/users")
        .json(&json!({
            "username": "user1",
            "email": "same@example.com"
        }))
        .await;

    // Try to create second user with same email
    let response = server
        .post("/users")
        .json(&json!({
            "username": "user2",
            "email": "same@example.com"
        }))
        .await;

    response.assert_status(StatusCode::CONFLICT);
    let body: Value = response.json();
    assert_eq!(body["error"], "Conflict");
    assert_eq!(body["details"], "Email already exists");
}

// ============================================================
// PUT /users/:id - Update user
// ============================================================

/// Test PUT /users/:id updates username
#[tokio::test]
async fn test_update_user_username() {
    let server = common::create_test_server().await;

    // Create a user
    let create_response = server
        .post("/users")
        .json(&json!({
            "username": "original",
            "email": "update@example.com"
        }))
        .await;

    let created: Value = create_response.json();
    let user_id = created["data"]["id"].as_i64().unwrap();

    // Update the user
    let response = server
        .put(&format!("/users/{}", user_id))
        .json(&json!({
            "username": "updated"
        }))
        .await;

    response.assert_status(StatusCode::OK);
    let body: Value = response.json();

    // Check { "data": { ... } } format
    assert_eq!(body["data"]["username"], "updated");
    assert_eq!(body["data"]["email"], "update@example.com"); // Email unchanged
}

/// Test PUT /users/:id returns 404 for non-existent user
#[tokio::test]
async fn test_update_user_not_found() {
    let server = common::create_test_server().await;

    let response = server
        .put("/users/999")
        .json(&json!({
            "username": "updated"
        }))
        .await;

    response.assert_status(StatusCode::NOT_FOUND);
}

/// Test PUT /users/:id returns 409 when changing to existing email
#[tokio::test]
async fn test_update_user_duplicate_email() {
    let server = common::create_test_server().await;

    // Create two users
    server
        .post("/users")
        .json(&json!({
            "username": "user1",
            "email": "user1@example.com"
        }))
        .await;

    let create_response = server
        .post("/users")
        .json(&json!({
            "username": "user2",
            "email": "user2@example.com"
        }))
        .await;

    let user2: Value = create_response.json();
    let user2_id = user2["data"]["id"].as_i64().unwrap();

    // Try to update user2's email to user1's email
    let response = server
        .put(&format!("/users/{}", user2_id))
        .json(&json!({
            "email": "user1@example.com"
        }))
        .await;

    response.assert_status(StatusCode::CONFLICT);
}

// ============================================================
// DELETE /users/:id - Delete user
// ============================================================

/// Test DELETE /users/:id deletes user successfully
#[tokio::test]
async fn test_delete_user_success() {
    let server = common::create_test_server().await;

    // Create a user
    let create_response = server
        .post("/users")
        .json(&json!({
            "username": "todelete",
            "email": "delete@example.com"
        }))
        .await;

    let created: Value = create_response.json();
    let user_id = created["data"]["id"].as_i64().unwrap();

    // Delete the user
    let response = server.delete(&format!("/users/{}", user_id)).await;
    response.assert_status(StatusCode::NO_CONTENT);

    // Verify user is gone
    let get_response = server.get(&format!("/users/{}", user_id)).await;
    get_response.assert_status(StatusCode::NOT_FOUND);
}

/// Test DELETE /users/:id returns 404 for non-existent user
#[tokio::test]
async fn test_delete_user_not_found() {
    let server = common::create_test_server().await;

    let response = server.delete("/users/999").await;

    response.assert_status(StatusCode::NOT_FOUND);
}
