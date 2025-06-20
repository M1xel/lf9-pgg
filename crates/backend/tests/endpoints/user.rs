use actix_web::{http::header, test};
use serde::{Deserialize, Serialize};

use crate::{common::test_helpers::TestContext, create_test_app};

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
        let ctx: TestContext = TestContext::new();
        let db = crate::common::test_helpers::get_database().await;

        let app = create_test_app!();

        // Create JSON payload using TestContext's ID
        let user_data = serde_json::json!({
            "username": format!("user_{}", ctx.test_id),
            "name": format!("Test User {}", ctx.test_id),
            "password": "password123"
        });

        let resp = test::TestRequest::post()
            .uri("/api/v1/user")
            .insert_header(header::ContentType::json())
            .set_payload(user_data.to_string())
            .send_request(&app)
            .await;

        dbg!(&resp);
        let status = resp.status();

        assert!(
            status.is_success(),
            "Expected success status, got: {}",
            status
        );

        let user: RespCreateUser = test::read_body_json(resp).await;

        assert!(!user.name.is_empty());
        assert!(!user.username.is_empty());
        assert!(user.username.starts_with("user_test_"));
        assert!(user.name.starts_with("Test User"));
        assert!(status.is_success());

        let user_id = uuid::Uuid::parse_str(&user.id).unwrap();
        assert!(ctx.assert_user_exists(db, user_id).await);

        ctx.cleanup_all(db).await;
    }

    #[actix_web::test]
    async fn test_delete_user() {
        let ctx = TestContext::new();
        let db = crate::common::test_helpers::get_database().await;

        let app = create_test_app!();

        let user = ctx.create_user(db, None, None).await.unwrap();

        // Check if user exists before deletion
        assert!(ctx.assert_user_exists(db, user.id).await);

        // Delete the user via API
        let delete_resp = test::TestRequest::delete()
            .uri(&format!("/api/v1/user/{}", user.id))
            .send_request(&app)
            .await;
        let delete_status = delete_resp.status();

        dbg!(&delete_resp);

        let delete_message: String = test::read_body_json(delete_resp).await;
        assert_eq!(delete_message, format!("User {} deleted", user.id));
        assert!(
            delete_status.is_success(),
            "Failed to delete user with status: {:?}",
            delete_status
        );

        // Verify user no longer exists in database
        assert!(ctx.assert_user_not_exists(db, user.id).await);

        // Cleanup
        ctx.cleanup_all(db).await;
    }

    #[actix_web::test]
    async fn test_get_users() {
        let ctx = TestContext::new();
        let db = crate::common::test_helpers::get_database().await;

        let app = create_test_app!();

        // Create some test users
        let users = ctx.create_multiple_users(db, 3).await.unwrap();
        assert_eq!(users.len(), 3);

        // Test the API endpoint
        let resp = test::TestRequest::get()
            .uri("/api/v1/user")
            .send_request(&app)
            .await;

        let status = resp.status();
        let api_users: Vec<RespCreateUser> = test::read_body_json(resp).await;

        assert!(status.is_success());
        assert!(api_users.len() >= 3); // At least our 3 users (could be more from other tests)

        // Verify our users are in the response
        for user in &users {
            let found = api_users.iter().any(|api_user| {
                api_user.id == user.id.to_string()
                    && api_user.username == user.username
                    && api_user.name == user.name
            });
            assert!(found, "User {} not found in API response", user.username);
        }

        // Verify database consistency
        assert!(ctx.assert_user_count(db, 3).await);

        // Cleanup
        ctx.cleanup_all(db).await;
    }

    #[actix_web::test]
    async fn test_delete_nonexistent_user() {
        let ctx = TestContext::new();
        let db = crate::common::test_helpers::get_database().await;

        let app = create_test_app!();
        let fake_id = "00000000-0000-0000-0000-000000000000";

        // Verify the fake ID doesn't exist in database
        let fake_uuid = uuid::Uuid::parse_str(fake_id).unwrap();
        assert!(ctx.assert_user_not_exists(db, fake_uuid).await);

        // Try to delete non-existent user
        let resp = test::TestRequest::delete()
            .uri(&format!("/api/v1/user/{}", fake_id))
            .send_request(&app)
            .await;

        let status = resp.status();
        assert!(
            status.is_client_error() || status.is_server_error(),
            "Expected error for non-existent user, got: {}",
            status
        );

        // Verify it still doesn't exist
        assert!(ctx.assert_user_not_exists(db, fake_uuid).await);

        // Cleanup
        ctx.cleanup_all(db).await;
    }
}
