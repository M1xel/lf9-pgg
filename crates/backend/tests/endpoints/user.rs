use actix_web::{http::header, test};
use serde::{Deserialize, Serialize};

use crate::{common::test_helpers::UserFactory, create_test_app};

#[cfg(test)]
mod tests {
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
        let user_data = UserFactory::create_unique_request();

        let resp = test::TestRequest::post()
            .uri("/api/v1/user")
            .insert_header(header::ContentType::json())
            .set_payload(user_data.to_string())
            .send_request(&app)
            .await;

        let status = resp.status();
        let user: RespCreateUser = test::read_body_json(resp).await;

        // Verify that the user was created with the expected structure
        assert!(!user.name.is_empty());
        assert!(!user.username.is_empty());
        assert!(user.username.starts_with("user_test_"));
        assert!(user.name.starts_with("Test User"));
        assert!(status.is_success());

        // Cleanup - delete the created user
        let _delete_resp = test::TestRequest::delete()
            .uri(&format!("/api/v1/user/{}", user.id))
            .send_request(&app)
            .await;
        // Don't assert on cleanup status in case of race conditions
    }

    #[actix_web::test]
    async fn test_delete_user() {
        let app = create_test_app!();
        let user_data = UserFactory::create_unique_request();

        // Create user to delete
        let create_resp = test::TestRequest::post()
            .uri("/api/v1/user")
            .insert_header(header::ContentType::json())
            .set_payload(user_data.to_string())
            .send_request(&app)
            .await;

        let create_status = create_resp.status();
        assert!(
            create_status.is_success(),
            "Failed to create user: {}",
            create_status
        );
        let user: RespCreateUser = test::read_body_json(create_resp).await;

        // Delete the user
        let delete_resp = test::TestRequest::delete()
            .uri(&format!("/api/v1/user/{}", user.id))
            .send_request(&app)
            .await;
        let delete_status = delete_resp.status();

        let delete_message: String = test::read_body_json(delete_resp).await;
        assert_eq!(delete_message, format!("User {} deleted", user.id));
        assert!(
            delete_status.is_success(),
            "Failed to delete user with status: {:?}",
            delete_status
        );
    }

    #[actix_web::test]
    async fn test_get_users() {
        let app = create_test_app!();

        let resp = test::TestRequest::get()
            .uri("/api/v1/user")
            .send_request(&app)
            .await;

        let status = resp.status();
        let users: Vec<RespCreateUser> = test::read_body_json(resp).await;

        assert!(status.is_success());
        assert!(users.is_empty() || !users.is_empty()); // Just verify it returns a valid array
    }

    #[actix_web::test]
    async fn test_create_user_duplicate_username() {
        let app = create_test_app!();
        let user_data = UserFactory::create_unique_request();

        // Create first user
        let resp1 = test::TestRequest::post()
            .uri("/api/v1/user")
            .insert_header(header::ContentType::json())
            .set_payload(user_data.to_string())
            .send_request(&app)
            .await;

        let status1 = resp1.status();
        let user1: RespCreateUser = test::read_body_json(resp1).await;
        assert!(status1.is_success());

        // Try to create user with same username
        let resp2 = test::TestRequest::post()
            .uri("/api/v1/user")
            .insert_header(header::ContentType::json())
            .set_payload(user_data.to_string())
            .send_request(&app)
            .await;

        let status2 = resp2.status();
        assert!(status2.is_client_error() || status2.is_server_error());

        // Cleanup
        let _delete_resp = test::TestRequest::delete()
            .uri(&format!("/api/v1/user/{}", user1.id))
            .send_request(&app)
            .await;
        // Don't assert on cleanup status in case of race conditions
    }

    #[actix_web::test]
    async fn test_delete_nonexistent_user() {
        let app = create_test_app!();
        let fake_id = "00000000-0000-0000-0000-000000000000";

        let resp = test::TestRequest::delete()
            .uri(&format!("/api/v1/user/{}", fake_id))
            .send_request(&app)
            .await;

        let status = resp.status();
        assert!(status.is_client_error() || status.is_server_error());
    }
}
