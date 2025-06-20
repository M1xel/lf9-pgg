use crate::{common::test_helpers::TestContext, create_test_app, with_test_context};
use actix_web::test;
use backend::Database;

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_user_crud_operations() {
        with_test_context!(|ctx: TestContext, db: &Database| async move {
            // Create single user
            let user = ctx
                .create_user(
                    db,
                    Some("testuser".to_string()),
                    Some("Test User".to_string()),
                )
                .await
                .unwrap();
            assert_eq!(user.username, "testuser");
            assert_eq!(user.name, "Test User");

            // Assert user exists
            assert!(ctx.assert_user_exists(db, user.id).await);

            // Get user by ID
            let retrieved_user = ctx.get_user_by_id(db, user.id).await.unwrap().unwrap();
            assert_eq!(retrieved_user.id, user.id);

            // Create multiple users
            let users = ctx.create_multiple_users(db, 3).await.unwrap();
            assert_eq!(users.len(), 3);

            // Assert user count (initial user + 3 new users = 4)
            assert!(ctx.assert_user_count(db, 4).await);

            // Delete a user
            ctx.delete_user(db, user.id).await.unwrap();
            assert!(ctx.assert_user_not_exists(db, user.id).await);

            // All cleanup handled automatically
        });
    }

    #[actix_web::test]
    async fn test_project_crud_operations() {
        with_test_context!(|ctx: TestContext, db: &Database| async move {
            // Create project with custom name
            let project = ctx
                .create_project_with_name(db, "My Test Project".to_string())
                .await
                .unwrap();
            assert_eq!(project.name, "My Test Project");

            // Assert project exists
            assert!(ctx.assert_project_exists(db, &project.id).await);

            // Update project
            let updated = ctx
                .update_project(db, &project.id, "Updated Project Name".to_string())
                .await
                .unwrap();
            assert_eq!(updated.name, "Updated Project Name");

            // Assert project name changed
            assert!(
                ctx.assert_project_name(db, &project.id, "Updated Project Name")
                    .await
            );

            // Create multiple projects
            let projects = ctx.create_multiple_projects(db, 2).await.unwrap();
            assert_eq!(projects.len(), 2);

            // Assert project count (1 original + 2 new = 3)
            assert!(ctx.assert_project_count(db, 3).await);

            // All cleanup automatic
        });
    }

    #[actix_web::test]
    async fn test_authentication_operations() {
        with_test_context!(|ctx: TestContext, db: &Database| async move {
            // Create authenticated user
            let (user, password) = ctx
                .create_authenticated_user(
                    db,
                    Some("authuser".to_string()),
                    Some("securepass123".to_string()),
                )
                .await
                .unwrap();

            // Test successful login
            assert!(
                ctx.assert_user_can_login(db, "authuser", "securepass123")
                    .await
            );

            // Test login returns correct user ID
            assert!(
                ctx.assert_user_login_returns_correct_id(db, "authuser", "securepass123", user.id)
                    .await
            );

            // Test failed login with wrong password
            assert!(
                ctx.assert_user_cannot_login(db, "authuser", "wrongpassword")
                    .await
            );

            // Test multiple invalid login attempts
            let invalid_results = ctx.test_invalid_login_attempts(db, "authuser").await;
            assert!(invalid_results.iter().all(|&result| result)); // All should fail (return true)

            // Create and verify multiple auth users
            let auth_users = ctx
                .create_multiple_authenticated_users(db, 2)
                .await
                .unwrap();
            assert_eq!(auth_users.len(), 2);

            for (user, pwd) in &auth_users {
                assert!(ctx.assert_user_can_login(db, &user.username, pwd).await);
            }

            // All cleanup automatic
        });
    }

    #[actix_web::test]
    async fn test_mixed_operations_with_api() {
        with_test_context!(|ctx: TestContext, db: &Database| async move {
            let app = create_test_app!();

            // Create user via helper
            let user = ctx.create_user(db, None, None).await.unwrap();

            // Create project via helper
            let project = ctx.create_project(db, None).await.unwrap();

            // Test API endpoints
            let user_resp = test::TestRequest::get()
                .uri(&format!("/api/v1/user/{}", user.id))
                .send_request(&app)
                .await;
            assert!(user_resp.status().is_success());

            let projects_resp = test::TestRequest::get()
                .uri("/api/v1/project")
                .send_request(&app)
                .await;
            assert!(projects_resp.status().is_success());

            // Assert both exist in database
            assert!(ctx.assert_user_exists(db, user.id).await);
            assert!(ctx.assert_project_exists(db, &project.id).await);

            // All cleanup automatic
        });
    }
}
