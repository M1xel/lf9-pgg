use actix_session::Session;
use actix_web::{
    HttpRequest, HttpResponse, Responder, post,
    web::{self, ServiceConfig},
};
use log::debug;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    Database,
    error::{ApiError, MessageResponse},
};

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    /// Username for authentication
    pub username: String,
    /// Password for authentication
    pub password: String,
}

pub fn setup(cfg: &mut ServiceConfig) {
    cfg.service(login).service(logout);
}

#[utoipa::path(
    post,
    path = "/api/v1/login",
    tag = "auth",
    summary = "User login",
    description = "Authenticate a user with username and password",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = MessageResponse, content_type = "application/json"),
        (status = 400, description = "Invalid credentials"),
        (status = 409, description = "User already logged in"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/login")]
pub async fn login(
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

    Ok(HttpResponse::Ok().json(MessageResponse::new("Login successful")))
}

#[utoipa::path(
    post,
    path = "/api/v1/logout",
    tag = "auth",
    summary = "User logout",
    description = "Log out the currently authenticated user and clear session",
    responses(
        (status = 200, description = "Logout successful", body = MessageResponse, content_type = "application/json"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/logout")]
pub async fn logout(session: Session, request: HttpRequest) -> Result<impl Responder, ApiError> {
    debug!("request cookies: {:?}", request.cookies());
    debug!("Session entries: {:?}", session.entries());
    session.purge();
    debug!("Session entries after purge: {:?}", session.entries());
    Ok(HttpResponse::Ok().json(MessageResponse::new("Logged out successfully")))
}
