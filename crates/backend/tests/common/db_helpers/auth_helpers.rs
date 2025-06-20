use crate::common::test_helpers::TestContext;
use backend::{Database, db::entity};
use uuid::Uuid;

impl TestContext {
    pub async fn create_user_with_auth(
        &self,
        db: &Database,
        username: Option<String>,
        name: Option<String>,
        password: String,
    ) -> Result<entity::user::Model, backend::error::ApiError> {
        self.create_user_with_password(db, username, name, password)
            .await
    }

    pub async fn verify_user_login(
        &self,
        db: &Database,
        username: &str,
        password: &str,
    ) -> Result<Uuid, backend::error::ApiError> {
        db.verify_local_user(username, password).await
    }

    pub async fn create_authenticated_user(
        &self,
        db: &Database,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<(entity::user::Model, String), backend::error::ApiError> {
        let test_id = &self.test_id;
        let username = username.unwrap_or_else(|| format!("auth_user_{}", test_id));
        let password = password.unwrap_or_else(|| "test_password_123".to_string());

        let user = self
            .create_user_with_password(db, Some(username.clone()), None, password.clone())
            .await?;

        Ok((user, password))
    }

    pub async fn create_multiple_authenticated_users(
        &self,
        db: &Database,
        count: usize,
    ) -> Result<Vec<(entity::user::Model, String)>, backend::error::ApiError> {
        let mut users = Vec::new();

        for i in 0..count {
            let username = format!("auth_user_{}_{}", self.test_id, i);
            let password = format!("password_{}", i);
            let user_data = self
                .create_authenticated_user(db, Some(username), Some(password.clone()))
                .await?;
            users.push(user_data);
        }

        Ok(users)
    }

    pub async fn assert_user_can_login(
        &self,
        db: &Database,
        username: &str,
        password: &str,
    ) -> bool {
        match self.verify_user_login(db, username, password).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn assert_user_cannot_login(
        &self,
        db: &Database,
        username: &str,
        password: &str,
    ) -> bool {
        !self.assert_user_can_login(db, username, password).await
    }

    pub async fn assert_user_login_returns_correct_id(
        &self,
        db: &Database,
        username: &str,
        password: &str,
        expected_id: Uuid,
    ) -> bool {
        match self.verify_user_login(db, username, password).await {
            Ok(id) => id == expected_id,
            Err(_) => false,
        }
    }

    pub async fn test_invalid_login_attempts(&self, db: &Database, username: &str) -> Vec<bool> {
        let invalid_passwords = vec!["wrong_password", "", "123", "password"];
        let mut results = Vec::new();

        for password in invalid_passwords {
            let can_login = self.assert_user_can_login(db, username, password).await;
            results.push(!can_login); // We expect these to fail, so invert the result
        }

        results
    }

    pub async fn create_user_and_verify_auth(
        &self,
        db: &Database,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<(entity::user::Model, bool), backend::error::ApiError> {
        let (user, pwd) = self
            .create_authenticated_user(db, username, password)
            .await?;
        let can_login = self.assert_user_can_login(db, &user.username, &pwd).await;

        Ok((user, can_login))
    }
}
