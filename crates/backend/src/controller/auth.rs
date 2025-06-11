use actix_session::Session;
use actix_web::{
    HttpRequest, HttpResponse, Responder, post,
    web::{self, ServiceConfig},
};
use log::debug;
use serde::Deserialize;

use crate::{Database, error::ApiError};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

pub fn setup(cfg: &mut ServiceConfig) {
    cfg.service(login).service(logout);
}

#[post("/login")]
async fn login(
    db: web::Data<Database>,
    login_request: web::Json<LoginRequest>,
    session: Session,
) -> Result<impl Responder, ApiError> {
    let login_request = login_request.into_inner();

    let user_id = db
        .verify_local_user(&login_request.username, &login_request.password)
        .await?;

    if session.get::<String>("user").is_ok() {
        return Err(ApiError::AlreadyLoggedIn);
    }

    session.insert("user", user_id)?;

    Ok(HttpResponse::Ok())
}

#[post("/logout")]
async fn logout(session: Session, request: HttpRequest) -> Result<impl Responder, ApiError> {
    debug!("request cookies: {:?}", request.cookies());
    debug!("Session entries: {:?}", session.entries());
    session.purge();
    debug!("Session entries after purge: {:?}", session.entries());
    Ok(HttpResponse::Ok().body("Logged out successfully"))
}
