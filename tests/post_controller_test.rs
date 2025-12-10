// tests/post_controller_test.rs
// Tests pour le PostController avec nested objects

use axum::http::StatusCode;
use serde_json::json;

mod common;

// ============================================================================
// LIST POSTS
// ============================================================================

#[tokio::test]
async fn test_list_posts_returns_empty_array() {
    let server = common::create_test_server().await;

    let response = server.get("/posts").await;

    response.assert_status(StatusCode::OK);
    let body: serde_json::Value = response.json();

    assert!(body["data"].is_array());
    assert_eq!(body["data"].as_array().unwrap().len(), 0);
    assert_eq!(body["meta"]["total"], 0);
}

// ============================================================================
// CREATE POST WITH NESTED OBJECTS
// ============================================================================

#[tokio::test]
async fn test_create_post_minimal() {
    let server = common::create_test_server().await;

    // Créer d'abord un utilisateur
    let user_response = server
        .post("/users")
        .json(&json!({
            "username": "author",
            "email": "author@test.com"
        }))
        .await;
    let user: serde_json::Value = user_response.json();
    let user_id = user["data"]["id"].as_i64().unwrap();

    // Créer un post avec le minimum requis
    let response = server
        .post("/posts")
        .json(&json!({
            "title": "Mon premier post",
            "content": "Contenu du post avec au moins 10 caractères",
            "author_id": user_id
        }))
        .await;

    response.assert_status(StatusCode::CREATED);

    let body: serde_json::Value = response.json();
    assert_eq!(body["data"]["title"], "Mon premier post");
    assert_eq!(body["data"]["published"], false);

    // Vérifier que l'auteur est inclus (nested)
    assert_eq!(body["data"]["author"]["id"], user_id);
    assert_eq!(body["data"]["author"]["username"], "author");

    // Vérifier les metadata par défaut
    assert!(body["data"]["metadata"]["tags"].is_array());
    assert_eq!(body["data"]["metadata"]["tags"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_create_post_with_full_nested_objects() {
    let server = common::create_test_server().await;

    // Créer un utilisateur
    let user_response = server
        .post("/users")
        .json(&json!({
            "username": "blogger",
            "email": "blogger@test.com"
        }))
        .await;
    let user: serde_json::Value = user_response.json();
    let user_id = user["data"]["id"].as_i64().unwrap();

    // Créer un post avec tous les nested objects
    let response = server
        .post("/posts")
        .json(&json!({
            "title": "Article complet avec metadata",
            "content": "Ceci est un article complet avec toutes les métadonnées imbriquées pour tester le système.",
            "author_id": user_id,
            "published": true,
            "metadata": {
                "tags": [
                    { "name": "rust", "color": "#DEA584" },
                    { "name": "api", "color": "#3178C6" },
                    { "name": "tutorial" }
                ],
                "seo": {
                    "meta_title": "Article complet | Mon Blog",
                    "meta_description": "Description SEO pour les moteurs de recherche",
                    "keywords": ["rust", "api", "tutorial", "backend"]
                },
                "settings": {
                    "allow_comments": true,
                    "featured": true,
                    "reading_time_minutes": 5
                }
            }
        }))
        .await;

    response.assert_status(StatusCode::CREATED);

    let body: serde_json::Value = response.json();

    // Vérifier le post
    assert_eq!(body["data"]["title"], "Article complet avec metadata");
    assert_eq!(body["data"]["published"], true);

    // Vérifier l'auteur (nested level 1)
    assert_eq!(body["data"]["author"]["username"], "blogger");

    // Vérifier les tags (nested level 2)
    let tags = body["data"]["metadata"]["tags"].as_array().unwrap();
    assert_eq!(tags.len(), 3);
    assert_eq!(tags[0]["name"], "rust");
    assert_eq!(tags[0]["color"], "#DEA584");
    assert_eq!(tags[2]["name"], "tutorial");
    assert!(tags[2]["color"].is_null()); // Pas de couleur fournie

    // Vérifier le SEO (nested level 2)
    assert_eq!(
        body["data"]["metadata"]["seo"]["meta_title"],
        "Article complet | Mon Blog"
    );
    let keywords = body["data"]["metadata"]["seo"]["keywords"].as_array().unwrap();
    assert_eq!(keywords.len(), 4);

    // Vérifier les settings (nested level 2)
    assert_eq!(body["data"]["metadata"]["settings"]["allow_comments"], true);
    assert_eq!(body["data"]["metadata"]["settings"]["featured"], true);
    assert_eq!(body["data"]["metadata"]["settings"]["reading_time_minutes"], 5);
}

// ============================================================================
// VALIDATION NESTED OBJECTS
// ============================================================================

#[tokio::test]
async fn test_create_post_validation_title_too_short() {
    let server = common::create_test_server().await;

    let response = server
        .post("/posts")
        .json(&json!({
            "title": "AB",  // Trop court (min 3)
            "content": "Contenu valide avec plus de 10 caractères",
            "author_id": 1
        }))
        .await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_create_post_validation_content_too_short() {
    let server = common::create_test_server().await;

    let response = server
        .post("/posts")
        .json(&json!({
            "title": "Titre valide",
            "content": "Court",  // Trop court (min 10)
            "author_id": 1
        }))
        .await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_create_post_validation_nested_tag_too_long() {
    let server = common::create_test_server().await;

    // Créer un utilisateur
    let user_response = server
        .post("/users")
        .json(&json!({
            "username": "tester",
            "email": "tester@test.com"
        }))
        .await;
    let user: serde_json::Value = user_response.json();
    let user_id = user["data"]["id"].as_i64().unwrap();

    let response = server
        .post("/posts")
        .json(&json!({
            "title": "Titre valide",
            "content": "Contenu valide avec plus de 10 caractères",
            "author_id": user_id,
            "metadata": {
                "tags": [
                    { "name": "Ce tag est beaucoup trop long et dépasse les 50 caractères autorisés par la validation" }
                ]
            }
        }))
        .await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_create_post_author_not_found() {
    let server = common::create_test_server().await;

    let response = server
        .post("/posts")
        .json(&json!({
            "title": "Post sans auteur",
            "content": "Contenu du post avec au moins 10 caractères",
            "author_id": 9999  // N'existe pas
        }))
        .await;

    response.assert_status(StatusCode::NOT_FOUND);
}

// ============================================================================
// GET POST
// ============================================================================

#[tokio::test]
async fn test_get_post_success() {
    let server = common::create_test_server().await;

    // Créer utilisateur + post
    let user_response = server
        .post("/users")
        .json(&json!({
            "username": "reader",
            "email": "reader@test.com"
        }))
        .await;
    let user: serde_json::Value = user_response.json();
    let user_id = user["data"]["id"].as_i64().unwrap();

    let post_response = server
        .post("/posts")
        .json(&json!({
            "title": "Post à lire",
            "content": "Contenu du post à récupérer",
            "author_id": user_id,
            "metadata": {
                "tags": [{ "name": "test" }]
            }
        }))
        .await;
    let post: serde_json::Value = post_response.json();
    let post_id = post["data"]["id"].as_i64().unwrap();

    // Récupérer le post
    let response = server.get(&format!("/posts/{}", post_id)).await;

    response.assert_status(StatusCode::OK);

    let body: serde_json::Value = response.json();
    assert_eq!(body["data"]["title"], "Post à lire");
    assert_eq!(body["data"]["author"]["username"], "reader");
    assert_eq!(body["data"]["metadata"]["tags"][0]["name"], "test");
}

#[tokio::test]
async fn test_get_post_not_found() {
    let server = common::create_test_server().await;

    let response = server.get("/posts/9999").await;

    response.assert_status(StatusCode::NOT_FOUND);
}

// ============================================================================
// UPDATE POST
// ============================================================================

#[tokio::test]
async fn test_update_post_title_only() {
    let server = common::create_test_server().await;

    // Setup
    let user_response = server
        .post("/users")
        .json(&json!({
            "username": "editor",
            "email": "editor@test.com"
        }))
        .await;
    let user: serde_json::Value = user_response.json();
    let user_id = user["data"]["id"].as_i64().unwrap();

    let post_response = server
        .post("/posts")
        .json(&json!({
            "title": "Titre original",
            "content": "Contenu original du post",
            "author_id": user_id
        }))
        .await;
    let post: serde_json::Value = post_response.json();
    let post_id = post["data"]["id"].as_i64().unwrap();

    // Update uniquement le titre
    let response = server
        .put(&format!("/posts/{}", post_id))
        .json(&json!({
            "title": "Nouveau titre modifié"
        }))
        .await;

    response.assert_status(StatusCode::OK);

    let body: serde_json::Value = response.json();
    assert_eq!(body["data"]["title"], "Nouveau titre modifié");
    assert_eq!(body["data"]["content"], "Contenu original du post"); // Inchangé
}

#[tokio::test]
async fn test_update_post_metadata() {
    let server = common::create_test_server().await;

    // Setup
    let user_response = server
        .post("/users")
        .json(&json!({
            "username": "updater",
            "email": "updater@test.com"
        }))
        .await;
    let user: serde_json::Value = user_response.json();
    let user_id = user["data"]["id"].as_i64().unwrap();

    let post_response = server
        .post("/posts")
        .json(&json!({
            "title": "Post avec metadata",
            "content": "Contenu du post avec metadata",
            "author_id": user_id,
            "metadata": {
                "tags": [{ "name": "old-tag" }]
            }
        }))
        .await;
    let post: serde_json::Value = post_response.json();
    let post_id = post["data"]["id"].as_i64().unwrap();

    // Update les metadata
    let response = server
        .put(&format!("/posts/{}", post_id))
        .json(&json!({
            "metadata": {
                "tags": [
                    { "name": "new-tag-1", "color": "#FF0000" },
                    { "name": "new-tag-2", "color": "#00FF00" }
                ],
                "seo": {
                    "meta_title": "Nouveau SEO title"
                }
            }
        }))
        .await;

    response.assert_status(StatusCode::OK);

    let body: serde_json::Value = response.json();
    let tags = body["data"]["metadata"]["tags"].as_array().unwrap();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0]["name"], "new-tag-1");
    assert_eq!(body["data"]["metadata"]["seo"]["meta_title"], "Nouveau SEO title");
}

// ============================================================================
// DELETE POST
// ============================================================================

#[tokio::test]
async fn test_delete_post_success() {
    let server = common::create_test_server().await;

    // Setup
    let user_response = server
        .post("/users")
        .json(&json!({
            "username": "deleter",
            "email": "deleter@test.com"
        }))
        .await;
    let user: serde_json::Value = user_response.json();
    let user_id = user["data"]["id"].as_i64().unwrap();

    let post_response = server
        .post("/posts")
        .json(&json!({
            "title": "Post à supprimer",
            "content": "Ce post sera supprimé",
            "author_id": user_id
        }))
        .await;
    let post: serde_json::Value = post_response.json();
    let post_id = post["data"]["id"].as_i64().unwrap();

    // Delete
    let response = server.delete(&format!("/posts/{}", post_id)).await;
    response.assert_status(StatusCode::NO_CONTENT);

    // Vérifier qu'il n'existe plus
    let get_response = server.get(&format!("/posts/{}", post_id)).await;
    get_response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_post_not_found() {
    let server = common::create_test_server().await;

    let response = server.delete("/posts/9999").await;
    response.assert_status(StatusCode::NOT_FOUND);
}

// ============================================================================
// LIST WITH PAGINATION
// ============================================================================

#[tokio::test]
async fn test_list_posts_with_pagination() {
    let server = common::create_test_server().await;

    // Créer un utilisateur
    let user_response = server
        .post("/users")
        .json(&json!({
            "username": "bulk",
            "email": "bulk@test.com"
        }))
        .await;
    let user: serde_json::Value = user_response.json();
    let user_id = user["data"]["id"].as_i64().unwrap();

    // Créer 5 posts
    for i in 1..=5 {
        server
            .post("/posts")
            .json(&json!({
                "title": format!("Post numéro {}", i),
                "content": format!("Contenu du post numéro {}", i),
                "author_id": user_id
            }))
            .await;
    }

    // Récupérer page 1 avec 2 items
    let response = server.get("/posts?page=1&per_page=2").await;

    response.assert_status(StatusCode::OK);

    let body: serde_json::Value = response.json();
    assert_eq!(body["data"].as_array().unwrap().len(), 2);
    assert_eq!(body["meta"]["total"], 5);
    assert_eq!(body["meta"]["page"], 1);
    assert_eq!(body["meta"]["per_page"], 2);
    assert_eq!(body["meta"]["total_pages"], 3);
}
