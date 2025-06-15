use crate::{Database, entity, error::ApiError};
use actix_web::{Responder, delete, get, post, put, web};
use serde::Deserialize;
use validator::Validate;
use utoipa::ToSchema;

pub fn setup(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_users)
        .service(get_user)
        .service(create_user)
        .service(delete_user);
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateUser {
    #[validate(length(min = 4))]
    /// Username (minimum 4 characters)
    username: String,
    /// Full name of the user
    name: String,
    #[validate(length(min = 8))]
    /// Password (minimum 8 characters)
    password: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/user",
    tag = "users",
    summary = "Get all users",
    description = "Retrieve a list of all users",
    responses(
        (status = 200, description = "List of users retrieved successfully", body = Vec<entity::user::Model>),
        (status = 500, description = "Internal server error", body = String)
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
        (status = 200, description = "User retrieved successfully", body = entity::user::Model),
        (status = 404, description = "User not found", body = String),
        (status = 500, description = "Internal server error", body = String)
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
        (status = 200, description = "User created successfully", body = entity::user::Model),
        (status = 400, description = "Invalid request data or validation error", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[post("")]
async fn create_user(
    db: web::Data<Database>,
    user: web::Json<CreateUser>,
) -> Result<web::Json<entity::user::Model>, ApiError> {
    let user = user.into_inner();
    let result = db
        .create_user(user.name, user.username, user.password)
        .await?;

    Ok(web::Json(result))
}

#[utoipa::path(
    put,
    path = "/api/v1/user",
    tag = "users",
    summary = "Update user (Not Implemented)",
    description = "Update user information - currently not implemented",
    responses(
        (status = 501, description = "Not implemented", body = String)
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
        (status = 200, description = "User deleted successfully", body = String),
        (status = 404, description = "User not found", body = String),
        (status = 500, description = "Internal server error", body = String)
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
