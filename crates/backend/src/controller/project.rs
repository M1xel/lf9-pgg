use actix_web::{Result, delete, get, post, put, web};
use uuid::Uuid;
use validator::Validate;

use crate::db::Database;
use crate::db::entity;
use crate::db::project::CreateProject;
use crate::error::ApiError;

pub fn setup(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_project)
        .service(get_projects)
        .service(create_project)
        .service(update_project)
        .service(delete_project);
}

#[utoipa::path(
    get,
    path = "/api/v1/project",
    tag = "projects",
    summary = "Get all projects",
    description = "Retrieve a list of all projects",
    responses(
        (status = 200, description = "List of projects retrieved successfully", body = Vec<entity::project::Model>, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = String, content_type = "application/json")
    )
)]
#[get("")]
async fn get_projects(
    db: web::Data<Database>,
) -> Result<web::Json<Vec<entity::project::Model>>, ApiError> {
    let projects = db.get_projects().await?;

    Ok(web::Json(projects))
}

#[utoipa::path(
    get,
    path = "/api/v1/project/{id}",
    tag = "projects",
    summary = "Get project by ID",
    description = "Retrieve a specific project by its ID",
    params(
        ("id" = String, Path, description = "Project ID")
    ),
    responses(
        (status = 200, description = "Project retrieved successfully", body = entity::project::Model, content_type = "application/json"),
        (status = 404, description = "Project not found", body = String, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = String, content_type = "application/json")
    )
)]
#[get("/{id}")]
async fn get_project(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<web::Json<entity::project::Model>, ApiError> {
    let id = path.into_inner();

    let project = db.get_project(&id).await?;

    Ok(web::Json(project.unwrap()))
}

#[utoipa::path(
    post,
    path = "/api/v1/project",
    tag = "projects",
    summary = "Create a new project",
    description = "Create a new project with the provided details",
    request_body = CreateProject,
    responses(
        (status = 200, description = "Project created successfully", body = entity::project::Model, content_type = "application/json"),
        (status = 400, description = "Invalid request data or validation error", body = String, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = String, content_type = "application/json")
    )
)]
#[post("")]
async fn create_project(
    db: web::Data<Database>,
    create_project: web::Json<CreateProject>,
) -> Result<web::Json<entity::project::Model>, ApiError> {
    create_project.validate()?;
    let result = db.create_project(create_project.into_inner()).await?;

    Ok(web::Json(result))
}

#[utoipa::path(
    put,
    path = "/api/v1/project/{id}",
    tag = "projects",
    summary = "Update project",
    description = "Update an existing project by its ID",
    params(
        ("id" = String, Path, description = "Project ID to update")
    ),
    request_body = CreateProject,
    responses(
        (status = 200, description = "Project updated successfully", body = entity::project::Model, content_type = "application/json"),
        (status = 400, description = "Invalid request data or validation error", body = String, content_type = "application/json"),
        (status = 404, description = "Project not found", body = String, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = String, content_type = "application/json")
    )
)]
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

#[utoipa::path(
    delete,
    path = "/api/v1/project/{id}",
    tag = "projects",
    summary = "Delete project",
    description = "Delete a project by its ID",
    params(
        ("id" = String, Path, description = "Project ID to delete")
    ),
    responses(
        (status = 200, description = "Project deleted successfully", body = String, content_type = "application/json"),
        (status = 404, description = "Project not found", body = String, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = String, content_type = "application/json")
    )
)]
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
