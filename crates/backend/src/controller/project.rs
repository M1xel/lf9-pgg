use actix_web::{delete, get, post, put, web, Result};
use uuid::Uuid;
use validator::Validate;

use crate::db::project::CreateProject;
use crate::db::Database;
use crate::entity;
use crate::error::ApiError;

pub fn setup(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_project)
        .service(get_projects)
        .service(create_project)
        .service(update_project)
        .service(delete_project);
}

#[get("")]
async fn get_projects(
    db: web::Data<Database>,
) -> Result<web::Json<Vec<entity::project::Model>>, ApiError> {
    let projects = db.get_projects().await?;

    Ok(web::Json(projects))
}

#[get("/{id}")]
async fn get_project(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<web::Json<entity::project::Model>, ApiError> {
    let id = path.into_inner();

    let project = db.get_project(&id).await?;

    Ok(web::Json(project.unwrap()))
}

#[post("")]
async fn create_project(
    db: web::Data<Database>,
    create_project: web::Json<CreateProject>,
) -> Result<web::Json<entity::project::Model>, ApiError> {
    create_project.validate()?;
    let result = db.create_project(create_project.into_inner()).await?;

    Ok(web::Json(result))
}

#[put("/{id}")]
async fn update_project(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
    update_project: web::Json<CreateProject>,
) -> Result<web::Json<entity::project::Model>, ApiError> {
    let updated_project = db
        .update_project(&path, update_project.into_inner())
        .await?;

    Ok(web::Json(updated_project))
}

#[delete("/{id}")]
async fn delete_project(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<web::Json<String>, ApiError> {
    let id = path.into_inner();
    let result = db.delete_project(&id).await?;

    Ok(web::Json(format!(
        "Successfully deleted {} project/s with the id: {}",
        result.rows_affected, id
    )))
}
