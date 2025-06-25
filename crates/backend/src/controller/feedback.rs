use actix_web::{HttpResponse, Responder, delete, get, post, web::ServiceConfig};

use crate::error::{ApiError, MessageResponse};

pub fn setup(cfg: &mut ServiceConfig) {
    cfg.service(get_feedback_form)
        .service(submit_feedback)
        .service(get_feedback_status)
        .service(reset_feedback);
}

#[utoipa::path(get, path = "/api/v1/feedback/{token}", tag = "feedback")]
#[get("/feedback/{token}")]
pub async fn get_feedback_form() -> Result<impl Responder, ApiError> {
    // TODO: Implement feedback form retrieval
    // 1. Validate token exists and is not completed
    // 2. Get project template and group information
    // 3. Return form structure

    Ok(HttpResponse::Ok().json("{}"))
}

#[utoipa::path(post, path = "/api/v1/feedback/{token}", tag = "feedback")]
#[post("/feedback/{token}")]
pub async fn submit_feedback() -> Result<impl Responder, ApiError> {
    // TODO: Implement feedback submission
    // 1. Validate token and ensure not already completed
    // 2. Store feedback responses
    // 3. Mark as completed with timestamp

    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "Feedback submitted successfully".to_string(),
    }))
}

#[utoipa::path(get, path = "/api/v1/feedback/{token}/status", tag = "feedback")]
#[get("/feedback/{token}/status")]
pub async fn get_feedback_status() -> Result<impl Responder, ApiError> {
    // TODO: Implement status checking
    // 1. Get completion status for this token
    // 2. Check if all group members completed feedback
    // 3. Calculate final grade if group grade exists
    // 4. Get anonymized feedback received

    Ok(HttpResponse::Ok().json("{}"))
}

#[utoipa::path(delete, path = "/api/v1/feedback/{token}/reset", tag = "feedback")]
#[delete("/feedback/{token}/reset")]
pub async fn reset_feedback() -> Result<impl Responder, ApiError> {
    // TODO: Implement feedback reset
    // 1. Verify teacher authorization
    // 2. Reset completion status and clear responses
    // 3. Optionally regenerate token

    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "Feedback reset successfully".to_string(),
    }))
}
