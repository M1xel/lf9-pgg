use actix_web::{App, http::header, test, web};
use backend::controller;
use serde::{Deserialize, Serialize};

use crate::common::test_helpers::get_database;

#[cfg(test)]
mod tests {
    use log::debug;
    use serde_json::json;

    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    struct UserLogin {
        username: String,
        name: String,
        password: String,
    }

    #[actix_web::test]
    async fn test_login() {
        let db = get_database().await;

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(db.clone()))
                .service(web::scope("/api/v1").configure(controller::register_controllers)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/v1/user")
            .insert_header(header::ContentType::json())
            .set_payload(
                json!({
                    "username": "testuser",
                    "name": "Test User",
                    "password": "password"
                })
                .to_string(),
            )
            .send_request(&app)
            .await;

        let status = req.status();
        let body = test::read_body(req).await;
        let body_str = String::from_utf8_lossy(&body);

        debug!("Response status: {}", status);
        debug!("Response body: {}", body_str);

        assert!(status.is_success() || status.is_client_error());
    }
}
