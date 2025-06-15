use actix_web::{delete, get, post, put, Responder};

pub fn setup(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_groups)
        .service(get_groups_for_project)
        .service(create_group)
        .service(update_group)
        .service(delete_group);
}

#[utoipa::path(
    get,
    path = "/api/v1/group",
    tag = "groups",
    summary = "Get all groups (Not Implemented)",
    description = "Retrieve a list of all groups - currently not implemented",
    responses(
        (status = 501, description = "Not implemented", body = String)
    )
)]
#[get("")]
async fn get_groups() -> impl Responder {
    ""
}

#[utoipa::path(
    get,
    path = "/api/v1/group/{project}",
    tag = "groups",
    summary = "Get groups for project (Not Implemented)",
    description = "Retrieve groups for a specific project - currently not implemented",
    params(
        ("project" = String, Path, description = "Project ID")
    ),
    responses(
        (status = 501, description = "Not implemented", body = String)
    )
)]
#[get("/{project}")]
async fn get_groups_for_project() -> impl Responder {
    ""
}

#[utoipa::path(
    post,
    path = "/api/v1/group",
    tag = "groups",
    summary = "Create group (Not Implemented)",
    description = "Create a new group - currently not implemented",
    responses(
        (status = 501, description = "Not implemented", body = String)
    )
)]
#[post("")]
async fn create_group() -> impl Responder {
    ""
}

#[utoipa::path(
    put,
    path = "/api/v1/group",
    tag = "groups",
    summary = "Update group (Not Implemented)",
    description = "Update an existing group - currently not implemented",
    responses(
        (status = 501, description = "Not implemented", body = String)
    )
)]
#[put("")]
async fn update_group() -> impl Responder {
    ""
}

#[utoipa::path(
    delete,
    path = "/api/v1/group/{id}",
    tag = "groups",
    summary = "Delete group (Not Implemented)",
    description = "Delete a group by its ID - currently not implemented",
    params(
        ("id" = String, Path, description = "Group ID to delete")
    ),
    responses(
        (status = 501, description = "Not implemented", body = String)
    )
)]
#[delete("/{id}")]
async fn delete_group() -> impl Responder {
    ""
}
