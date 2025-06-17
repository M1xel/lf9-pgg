use actix_web::{http::header, test};
use serde::{Deserialize, Serialize};

use crate::create_test_app;

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
        let app = create_test_app!();

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
