use crate::{Database, db::entity, error::ApiError};
use actix_web::error::ErrorInternalServerError;
use actix_web::{Responder, delete, get, post, put, web};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn setup(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_users)
        .service(get_user)
        .service(create_user)
        .service(delete_user);
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateUser {
    #[validate(length(min = 4, max = 255))]
    /// Username (minimum 4 characters, maximum 255 characters)
    /// TODO: Don't allow spaces, only alphanumeric characters and underscores
    username: String,
    #[validate(length(min = 3))]
    /// Full name of the user (minimum 3 characters)
    name: String,
    #[validate(length(min = 8, max = 255))]
    /// Password (minimum 8 characters, maximum 255 characters)
    password: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/user",
    tag = "users",
    summary = "Get all users",
    description = "Retrieve a list of all users",
    responses(
        (status = 200, description = "List of users retrieved successfully", body = Vec<entity::user::Model>, content_type = "application/json",
        example = json!([
            {
                "id": "831195d1-01c4-4029-8284-349f5c41e398",
                "username": "MyAwesomeUsername",
                "name": "My Awesome Name",
            },
            {
                "id": "0024870c-ea5c-4927-802f-8e44fc57b098",
                "username": "AnotherUser",
                "name": "Another User",
            }
        ])),
        (status = 500, description = "Internal server error", body = String, content_type = "application/json")
    )
)]
#[get("")]
async fn get_users(
    db: web::Data<Database>,
) -> Result<web::Json<Vec<entity::user::Model>>, ApiError> {
    let users = db.get_users().await?;
    Ok(web::Json(users))
}

#[utoipa::path(
    get,
    path = "/api/v1/user/{id}",
    tag = "users",
    summary = "Get user by ID",
    description = "Retrieve a specific user by their ID",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User retrieved successfully", body = entity::user::Model, content_type = "application/json",
        example = json!({
            "id": "831195d1-01c4-4029-8284-349f5c41e398",
            "username": "MyAwesomeUsername",
            "name": "My Awesome Name",
        })),
        (status = 404, description = "User not found", body = String, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = String, content_type = "application/json")
    )
)]
#[get("/{id}")]
async fn get_user(
    db: web::Data<Database>,
    id: web::Path<uuid::Uuid>,
) -> Result<web::Json<entity::user::Model>, ApiError> {
    let user = db.get_user(id.into_inner()).await?;

    Ok(web::Json(user.unwrap()))
}

#[utoipa::path(
    post,
    path = "/api/v1/user",
    tag = "users",
    summary = "Create a new user",
    description = "Create a new user with username, name, and password",
    request_body = CreateUser,
    responses(
        (status = 200, description = "User created successfully", body = entity::user::Model, content_type = "application/json", 
        example = json!({
            "id": "831195d1-01c4-4029-8284-349f5c41e398",
            "username": "MyAwesomeUsername",
            "name": "My Awesome Name",
        })),
        (status = 400, description = "Invalid request data or validation error", body = String, content_type = "application/json"),
        (status = 409, description = "User already exists", body = String, content_type = "application/json", example = "User with username - MyAwesomeUsername - already exists"),
        (status = 500, description = "Internal server error", body = String, content_type = "application/json")
    )
)]
#[post("")]
async fn create_user(
    db: web::Data<Database>,
    user: web::Json<CreateUser>,
) -> Result<web::Json<entity::user::Model>, ApiError> {
    let user = user.into_inner();
    user.validate()
        .map_err(|e| ApiError::BadRequest(format!("\nValidation error: {}", e)))?;

    let username = user.username.clone();
    let result = db
        .create_user(user.name, user.username, user.password)
        .await;

    match result {
        Ok(result) => Ok(web::Json(result)),
        Err(e) => {
            if e.to_string().contains("user_username_key") {
                Err(ApiError::UserAlreadyExists(username))
            } else {
                Err(ApiError::InternalServerError(
                    "/user/ - create_user - Error: {e}".to_owned(),
                ))
            }
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/user",
    tag = "users",
    summary = "Update user (Not Implemented)",
    description = "Update user information - currently not implemented",
    responses(
        (status = 501, description = "Not implemented", body = String, content_type = "application/json")
    )
)]
#[put("")]
async fn update_user() -> impl Responder {
    ""
}

#[utoipa::path(
    delete,
    path = "/api/v1/user/{id}",
    tag = "users",
    summary = "Delete user",
    description = "Delete a user by their ID",
    params(
        ("id" = String, Path, description = "User ID to delete")
    ),
    responses(
        (status = 200, description = "User deleted successfully", body = String, content_type = "application/json", example = "User 123e4567-e89b-12d3-a456-426614174000 deleted"),
        (status = 404, description = "User not found", body = String, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = String, content_type = "application/json")
    )
)]
#[delete("/{id}")]
async fn delete_user(
    db: web::Data<Database>,
    id: web::Path<uuid::Uuid>,
) -> Result<web::Json<String>, ApiError> {
    let id = id.into_inner();
    db.delete_user(id).await?;
    Ok(web::Json(format!("User {} deleted", id)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_validation_create_user_struct_valid() {
        let user = CreateUser {
            username: "testuser".to_string(),
            name: "Test User".to_string(),
            password: "password123".to_string(),
        };
        let validation_result = user.validate();
        assert!(validation_result.is_ok());
    }

    #[actix_web::test]
    async fn test_validation_create_user_struct_username_invalid() {
        let user = CreateUser {
            username: "usr".to_string(), // too short
            name: "Test User".to_string(),
            password: "password".to_string(),
        };
        let validation_result = user.validate();
        assert!(validation_result.is_err());
    }

    #[actix_web::test]
    async fn test_validation_create_user_struct_username_too_long() {
        let user = CreateUser {
            username: "a".repeat(256), // too long
            name: "Test User".to_string(),
            password: "password123".to_string(),
        };
        let validation_result = user.validate();
        assert!(validation_result.is_err());
    }

    #[actix_web::test]
    async fn test_validation_create_user_struct_name_invalid() {
        let user = CreateUser {
            username: "testuser".to_string(),
            name: "".to_string(), // empty name
            password: "password123".to_string(),
        };
        let validation_result = user.validate();
        assert!(validation_result.is_err());
    }

    #[actix_web::test]
    async fn test_validation_create_user_struct_password_invalid() {
        let user = CreateUser {
            username: "testuser".to_string(),
            name: "Test User".to_string(),
            password: "pass".to_string(), // too short
        };
        let validation_result = user.validate();
        assert!(validation_result.is_err());
    }

    #[actix_web::test]
    async fn test_validation_create_user_struct_password_too_long() {
        let user = CreateUser {
            username: "testuser".to_string(),
            name: "Test User".to_string(),
            password: "a".repeat(256), // too long
        };
        let validation_result = user.validate();
        assert!(validation_result.is_err());
    }
}
