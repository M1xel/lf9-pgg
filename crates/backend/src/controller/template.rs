use actix_web::{delete, get, post, put, Responder};

pub fn setup(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_templates)
        .service(get_template)
        .service(create_template)
        .service(update_template)
        .service(delete_template);
}

// TODO
#[get("")]
async fn get_templates() -> impl Responder {
    ""
}

// TODO
#[get("/{id}")]
async fn get_template() -> impl Responder {
    ""
}

// TODO
#[post("")]
async fn create_template() -> impl Responder {
    ""
}

// TODO
#[put("")]
async fn update_template() -> impl Responder {
    ""
}

// TODO
#[delete("/{id}")]
async fn delete_template() -> impl Responder {
    ""
}
