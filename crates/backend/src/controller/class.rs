use actix_web::{delete, get, post, put, Responder};

pub fn setup(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_classes)
        .service(get_class)
        .service(create_class)
        .service(update_class)
        .service(delete_class);
}

// TODO
#[get("")]
async fn get_classes() -> impl Responder {
    ""
}

// TODO
#[get("/{id}")]
async fn get_class() -> impl Responder {
    ""
}

// TODO
#[post("")]
async fn create_class() -> impl Responder {
    ""
}

// TODO
#[put("")]
async fn update_class() -> impl Responder {
    ""
}

// TODO
#[delete("/{id}")]
async fn delete_class() -> impl Responder {
    ""
}
