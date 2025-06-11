use actix_web::web::{self, ServiceConfig};

mod auth;
mod class;
mod group;
mod project;
mod template;
mod user;

pub fn register_controllers(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/project").configure(project::setup))
        .service(web::scope("/group").configure(group::setup))
        .service(web::scope("/user").configure(user::setup))
        .service(web::scope("/class").configure(class::setup))
        .service(web::scope("/template").configure(template::setup))
        .service(web::scope("/auth").configure(auth::setup));
}
