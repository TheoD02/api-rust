// src/dto/user.rs
// Equivalent de: src/Dto/UserDto.php

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Request DTO for creating a new user
/// Equivalent de: CreateUserRequest en Symfony
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[schema(example = json!({
    "username": "johndoe",
    "email": "john@example.com"
}))]
pub struct CreateUserDto {
    /// Username (3-50 characters)
    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    #[schema(min_length = 3, max_length = 50)]
    pub username: String,

    /// Valid email address
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email must not exceed 255 characters"))]
    #[schema(format = "email", max_length = 255)]
    pub email: String,
}

/// Request DTO for updating a user
/// Equivalent de: UpdateUserRequest en Symfony
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[schema(example = json!({
    "username": "johndoe_updated",
    "email": "john.updated@example.com"
}))]
pub struct UpdateUserDto {
    /// Username (3-50 characters) - optional
    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    #[schema(min_length = 3, max_length = 50)]
    pub username: Option<String>,

    /// Valid email address - optional
    #[validate(email(message = "Invalid email format"))]
    #[schema(format = "email", max_length = 255)]
    pub email: Option<String>,
}

/// Response DTO for user data
/// Equivalent de: UserResponse en Symfony
#[derive(Debug, Serialize, ToSchema)]
#[schema(example = json!({
    "id": 1,
    "username": "johndoe",
    "email": "john@example.com",
    "created_at": "2024-01-15T10:30:00"
}))]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

impl From<crate::entities::user::Model> for UserResponse {
    fn from(user: crate::entities::user::Model) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
}
