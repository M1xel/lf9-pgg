use actix_web::{delete, get, post, put, Responder};

pub fn setup(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_groups)
        .service(get_groups_for_project)
        .service(create_group)
        .service(update_group)
        .service(delete_group);
}

// TODO
#[get("")]
async fn get_groups() -> impl Responder {
    ""
}

// TODO
#[get("/{project}")]
async fn get_groups_for_project() -> impl Responder {
    ""
}

// TODO
#[post("")]
async fn create_group() -> impl Responder {
    ""
}

// TODO
#[put("")]
async fn update_group() -> impl Responder {
    ""
}

// TODO
#[delete("/{id}")]
async fn delete_group() -> impl Responder {
    ""
}
