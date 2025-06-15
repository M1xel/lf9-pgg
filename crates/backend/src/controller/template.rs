use actix_web::{delete, get, post, put, Responder};

pub fn setup(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_templates)
        .service(get_template)
        .service(create_template)
        .service(update_template)
        .service(delete_template);
}

#[utoipa::path(
    get,
    path = "/api/v1/template",
    tag = "templates",
    summary = "Get all templates (Not Implemented)",
    description = "Retrieve a list of all templates - currently not implemented",
    responses(
        (status = 501, description = "Not implemented", body = String)
    )
)]
#[get("")]
async fn get_templates() -> impl Responder {
    ""
}

#[utoipa::path(
    get,
    path = "/api/v1/template/{id}",
    tag = "templates",
    summary = "Get template by ID (Not Implemented)",
    description = "Retrieve a specific template by its ID - currently not implemented",
    params(
        ("id" = String, Path, description = "Template ID")
    ),
    responses(
        (status = 501, description = "Not implemented", body = String)
    )
)]
#[get("/{id}")]
async fn get_template() -> impl Responder {
    ""
}

#[utoipa::path(
    post,
    path = "/api/v1/template",
    tag = "templates",
    summary = "Create template (Not Implemented)",
    description = "Create a new template - currently not implemented",
    responses(
        (status = 501, description = "Not implemented", body = String)
    )
)]
#[post("")]
async fn create_template() -> impl Responder {
    ""
}

#[utoipa::path(
    put,
    path = "/api/v1/template",
    tag = "templates",
    summary = "Update template (Not Implemented)",
    description = "Update an existing template - currently not implemented",
    responses(
        (status = 501, description = "Not implemented", body = String)
    )
)]
#[put("")]
async fn update_template() -> impl Responder {
    ""
}

#[utoipa::path(
    delete,
    path = "/api/v1/template/{id}",
    tag = "templates",
    summary = "Delete template (Not Implemented)",
    description = "Delete a template by its ID - currently not implemented",
    params(
        ("id" = String, Path, description = "Template ID to delete")
    ),
    responses(
        (status = 501, description = "Not implemented", body = String)
    )
)]
#[delete("/{id}")]
async fn delete_template() -> impl Responder {
    ""
}
