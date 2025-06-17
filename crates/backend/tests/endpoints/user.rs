use actix_web::{http::header, test};
use serde::{Deserialize, Serialize};

use crate::create_test_app;

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    struct RespCreateUser {
        id: String,
        username: String,
        name: String,
    }

    #[actix_web::test]
    async fn test_create_user() {
        let app = create_test_app!();

        let resp = test::TestRequest::post()
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

        let status = resp.status();
        let user: RespCreateUser = test::read_body_json(resp).await;

        assert!(user.name == "Test User");
        assert!(user.username == "testuser");

        assert!(status.is_success());
    }
}
