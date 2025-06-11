use crate::{Database, entity, error::ApiError};
use actix_web::{Responder, delete, get, post, put, web};
use serde::Deserialize;
use validator::Validate;

pub fn setup(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_users)
        .service(get_user)
        .service(create_user)
        .service(delete_user);
}

#[derive(Deserialize, Validate)]
struct CreateUser {
    #[validate(length(min = 4))]
    username: String,
    name: String,
    #[validate(length(min = 8))]
    password: String,
}

#[get("")]
async fn get_users(
    db: web::Data<Database>,
) -> Result<web::Json<Vec<entity::user::Model>>, ApiError> {
    let users = db.get_users().await?;
    Ok(web::Json(users))
}

#[get("/{id}")]
async fn get_user(
    db: web::Data<Database>,
    id: web::Path<uuid::Uuid>,
) -> Result<web::Json<entity::user::Model>, ApiError> {
    let user = db.get_user(id.into_inner()).await?;

    Ok(web::Json(user.unwrap()))
}

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

#[put("")]
async fn update_user() -> impl Responder {
    ""
}

#[delete("/{id}")]
async fn delete_user(
    db: web::Data<Database>,
    id: web::Path<uuid::Uuid>,
) -> Result<web::Json<String>, ApiError> {
    let id = id.into_inner();
    db.delete_user(id).await?;
    Ok(web::Json(format!("User {} deleted", id)))
}
