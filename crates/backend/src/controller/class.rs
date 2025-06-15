use actix_web::{delete, get, post, put, Responder};

pub fn setup(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_classes)
        .service(get_class)
        .service(create_class)
        .service(update_class)
        .service(delete_class);
}

#[utoipa::path(
    get,
    path = "/api/v1/class",
    tag = "classes",
    summary = "Get all classes (Not Implemented)",
    description = "Retrieve a list of all classes - currently not implemented",
    responses(
        (status = 501, description = "Not implemented", body = String)
    )
)]
#[get("")]
async fn get_classes() -> impl Responder {
    ""
}

#[utoipa::path(
    get,
    path = "/api/v1/class/{id}",
    tag = "classes",
    summary = "Get class by ID (Not Implemented)",
    description = "Retrieve a specific class by its ID - currently not implemented",
    params(
        ("id" = String, Path, description = "Class ID")
    ),
    responses(
        (status = 501, description = "Not implemented", body = String)
    )
)]
#[get("/{id}")]
async fn get_class() -> impl Responder {
    ""
}

#[utoipa::path(
    post,
    path = "/api/v1/class",
    tag = "classes",
    summary = "Create class (Not Implemented)",
    description = "Create a new class - currently not implemented",
    responses(
        (status = 501, description = "Not implemented", body = String)
    )
)]
#[post("")]
async fn create_class() -> impl Responder {
    ""
}

#[utoipa::path(
    put,
    path = "/api/v1/class",
    tag = "classes",
    summary = "Update class (Not Implemented)",
    description = "Update an existing class - currently not implemented",
    responses(
        (status = 501, description = "Not implemented", body = String)
    )
)]
#[put("")]
async fn update_class() -> impl Responder {
    ""
}

#[utoipa::path(
    delete,
    path = "/api/v1/class/{id}",
    tag = "classes",
    summary = "Delete class (Not Implemented)",
    description = "Delete a class by its ID - currently not implemented",
    params(
        ("id" = String, Path, description = "Class ID to delete")
    ),
    responses(
        (status = 501, description = "Not implemented", body = String)
    )
)]
#[delete("/{id}")]
async fn delete_class() -> impl Responder {
    ""
}
