use actix_web::web::{self, ServiceConfig};

// TODO: Refactor to use re-exports instead of making module public
pub mod auth;
pub mod class;
pub mod group;
pub mod project;
pub mod template;
pub mod user;

pub fn register_controllers(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/project").configure(project::setup))
        .service(web::scope("/group").configure(group::setup))
        .service(web::scope("/user").configure(user::setup))
        .service(web::scope("/class").configure(class::setup))
        .service(web::scope("/template").configure(template::setup))
        .service(web::scope("/auth").configure(auth::setup))
        .service(
            web::resource("/ok").to(|| async { actix_web::HttpResponse::Ok().body("available") }),
        );
}
