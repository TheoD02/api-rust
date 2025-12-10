// src/services/user_service.rs
// Equivalent de: src/Service/UserService.php

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
    ActiveValue::NotSet,
};
use tracing::{info, warn};

use crate::dto::{CreateUserDto, PaginationQuery, UpdateUserDto};
use crate::entities::user;
use crate::error::ServiceError;

/// Paginated result - returns entities, not DTOs
/// Transformation to DTO is done in the controller
pub struct PaginatedUsers {
    pub users: Vec<user::Model>,
    pub total: u64,
}

/// UserService - Business logic for user management
/// Returns entities (user::Model) - transformation to DTO is done in controllers
/// Uses ServiceError for business logic errors (no HTTP concepts)
#[derive(Clone)]
pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    /// Create a new UserService instance
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Find all users with pagination
    pub async fn find_all(&self, pagination: &PaginationQuery) -> Result<PaginatedUsers, ServiceError> {
        info!(page = pagination.page, per_page = pagination.per_page, "Fetching users");

        // Get total count
        let total = user::Entity::find()
            .count(&self.db)
            .await?;

        // Get paginated users
        let users = user::Entity::find()
            .order_by_asc(user::Column::Id)
            .offset(pagination.offset())
            .limit(pagination.limit())
            .all(&self.db)
            .await?;

        info!(
            count = users.len(),
            total = total,
            page = pagination.page,
            "Users fetched successfully"
        );

        Ok(PaginatedUsers { users, total })
    }

    /// Find a user by ID
    pub async fn find_by_id(&self, id: i32) -> Result<user::Model, ServiceError> {
        info!(user_id = id, "Fetching user by ID");

        let user = user::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| {
                warn!(user_id = id, "User not found");
                ServiceError::NotFound
            })?;

        info!(user_id = id, username = %user.username, "User found");
        Ok(user)
    }

    /// Create a new user
    pub async fn create(&self, dto: CreateUserDto) -> Result<user::Model, ServiceError> {
        info!(username = %dto.username, email = %dto.email, "Creating new user");

        // Check if email already exists
        let existing = user::Entity::find()
            .filter(user::Column::Email.eq(&dto.email))
            .one(&self.db)
            .await?;

        if existing.is_some() {
            warn!(email = %dto.email, "Email already exists");
            return Err(ServiceError::AlreadyExists("Email already exists".to_string()));
        }

        let new_user = user::ActiveModel {
            id: NotSet,
            username: Set(dto.username),
            email: Set(dto.email),
            created_at: Set(chrono::Utc::now().naive_utc()),
        };

        let user = new_user.insert(&self.db).await?;

        info!(user_id = user.id, username = %user.username, "User created successfully");
        Ok(user)
    }

    /// Update an existing user
    pub async fn update(&self, id: i32, dto: UpdateUserDto) -> Result<user::Model, ServiceError> {
        info!(user_id = id, "Updating user");

        // Find existing user
        let user = user::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| {
                warn!(user_id = id, "User not found for update");
                ServiceError::NotFound
            })?;

        // Check email uniqueness if changing
        if let Some(ref new_email) = dto.email {
            if new_email != &user.email {
                let existing = user::Entity::find()
                    .filter(user::Column::Email.eq(new_email))
                    .one(&self.db)
                    .await?;

                if existing.is_some() {
                    return Err(ServiceError::AlreadyExists("Email already exists".to_string()));
                }
            }
        }

        // Build update model
        let mut active_model: user::ActiveModel = user.into();

        if let Some(username) = dto.username {
            active_model.username = Set(username);
        }
        if let Some(email) = dto.email {
            active_model.email = Set(email);
        }

        let updated_user = active_model.update(&self.db).await?;

        info!(user_id = id, "User updated successfully");
        Ok(updated_user)
    }

    /// Delete a user
    pub async fn delete(&self, id: i32) -> Result<(), ServiceError> {
        info!(user_id = id, "Deleting user");

        let result = user::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;

        if result.rows_affected == 0 {
            warn!(user_id = id, "User not found for deletion");
            return Err(ServiceError::NotFound);
        }

        info!(user_id = id, "User deleted successfully");
        Ok(())
    }
}
