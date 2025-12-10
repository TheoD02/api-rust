// src/services/user_service.rs
// Equivalent de: src/Service/UserService.php

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
    ActiveValue::NotSet,
};
use tracing::{info, warn};

use crate::dto::{CreateUserDto, PaginationQuery, UpdateUserDto, UserResponse};
use crate::entities::user;
use crate::error::{ApiError, ApiResult};

/// Paginated result
pub struct PaginatedUsers {
    pub users: Vec<UserResponse>,
    pub total: u64,
}

/// UserService - Business logic for user management
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
    pub async fn find_all(&self, pagination: &PaginationQuery) -> ApiResult<PaginatedUsers> {
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

        Ok(PaginatedUsers {
            users: users.into_iter().map(UserResponse::from).collect(),
            total,
        })
    }

    /// Find a user by ID
    pub async fn find_by_id(&self, id: i32) -> ApiResult<UserResponse> {
        info!(user_id = id, "Fetching user by ID");

        let user = user::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| {
                warn!(user_id = id, "User not found");
                ApiError::NotFound
            })?;

        info!(user_id = id, username = %user.username, "User found");
        Ok(UserResponse::from(user))
    }

    /// Create a new user
    pub async fn create(&self, dto: CreateUserDto) -> ApiResult<UserResponse> {
        info!(username = %dto.username, email = %dto.email, "Creating new user");

        // Check if email already exists
        let existing = user::Entity::find()
            .filter(user::Column::Email.eq(&dto.email))
            .one(&self.db)
            .await?;

        if existing.is_some() {
            warn!(email = %dto.email, "Email already exists");
            return Err(ApiError::Conflict("Email already exists".to_string()));
        }

        let new_user = user::ActiveModel {
            id: NotSet,
            username: Set(dto.username),
            email: Set(dto.email),
            created_at: Set(chrono::Utc::now().naive_utc()),
        };

        let user = new_user.insert(&self.db).await?;

        info!(user_id = user.id, username = %user.username, "User created successfully");
        Ok(UserResponse::from(user))
    }

    /// Update an existing user
    pub async fn update(&self, id: i32, dto: UpdateUserDto) -> ApiResult<UserResponse> {
        info!(user_id = id, "Updating user");

        // Find existing user
        let user = user::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| {
                warn!(user_id = id, "User not found for update");
                ApiError::NotFound
            })?;

        // Check email uniqueness if changing
        if let Some(ref new_email) = dto.email {
            if new_email != &user.email {
                let existing = user::Entity::find()
                    .filter(user::Column::Email.eq(new_email))
                    .one(&self.db)
                    .await?;

                if existing.is_some() {
                    return Err(ApiError::Conflict("Email already exists".to_string()));
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
        Ok(UserResponse::from(updated_user))
    }

    /// Delete a user
    pub async fn delete(&self, id: i32) -> ApiResult<()> {
        info!(user_id = id, "Deleting user");

        let result = user::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;

        if result.rows_affected == 0 {
            warn!(user_id = id, "User not found for deletion");
            return Err(ApiError::NotFound);
        }

        info!(user_id = id, "User deleted successfully");
        Ok(())
    }
}
