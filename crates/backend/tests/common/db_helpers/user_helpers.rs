use crate::common::test_helpers::TestContext;
use backend::{Database, db::entity};
use uuid::Uuid;

impl TestContext {
    pub async fn create_user(
        &self,
        db: &Database,
        username: Option<String>,
        name: Option<String>,
    ) -> Result<entity::user::Model, backend::error::ApiError> {
        let test_id = &self.test_id;
        let username = username.unwrap_or_else(|| format!("user_{}", test_id));
        let name = name.unwrap_or_else(|| format!("Test User {}", test_id));
        let password = "password123".to_string();

        let user = db.create_user(name, username, password).await?;

        if let Ok(mut users) = self.created_users.lock() {
            users.push(user.id);
        }

        Ok(user)
    }

    pub async fn create_user_with_password(
        &self,
        db: &Database,
        username: Option<String>,
        name: Option<String>,
        password: String,
    ) -> Result<entity::user::Model, backend::error::ApiError> {
        let test_id = &self.test_id;
        let username = username.unwrap_or_else(|| format!("user_{}", test_id));
        let name = name.unwrap_or_else(|| format!("Test User {}", test_id));

        let user = db.create_user(name, username, password).await?;

        if let Ok(mut users) = self.created_users.lock() {
            users.push(user.id);
        }

        Ok(user)
    }

    pub async fn create_multiple_users(
        &self,
        db: &Database,
        count: usize,
    ) -> Result<Vec<entity::user::Model>, backend::error::ApiError> {
        let mut users = Vec::new();

        for i in 0..count {
            let username = format!("user_{}_{}", self.test_id, i);
            let name = format!("Test User {} {}", self.test_id, i);
            let user = self.create_user(db, Some(username), Some(name)).await?;
            users.push(user);
        }

        Ok(users)
    }

    pub async fn get_user_by_id(
        &self,
        db: &Database,
        id: Uuid,
    ) -> Result<Option<entity::user::Model>, backend::error::ApiError> {
        db.get_user(id).await
    }

    pub async fn get_all_users(
        &self,
        db: &Database,
    ) -> Result<Vec<entity::user::Model>, backend::error::ApiError> {
        db.get_users().await
    }

    pub async fn assert_user_exists(&self, db: &Database, id: Uuid) -> bool {
        match self.get_user_by_id(db, id).await {
            Ok(Some(_)) => true,
            _ => false,
        }
    }

    pub async fn assert_user_count(&self, db: &Database, expected: usize) -> bool {
        match self.get_all_users(db).await {
            Ok(users) => users.len() == expected,
            Err(_) => false,
        }
    }

    pub async fn assert_user_not_exists(&self, db: &Database, id: Uuid) -> bool {
        !self.assert_user_exists(db, id).await
    }

    pub async fn delete_user(
        &self,
        db: &Database,
        id: Uuid,
    ) -> Result<(), backend::error::ApiError> {
        db.delete_user(id).await?;

        if let Ok(mut users) = self.created_users.lock() {
            users.retain(|&user_id| user_id != id);
        }

        Ok(())
    }

    pub async fn cleanup_users(&self, db: &Database) {
        if let Ok(users) = self.created_users.lock() {
            for user_id in users.iter() {
                let _ = db.delete_user(*user_id).await;
            }
        }

        if let Ok(mut users) = self.created_users.lock() {
            users.clear();
        }
    }
}
