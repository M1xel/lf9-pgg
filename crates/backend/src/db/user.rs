use crate::error::ApiError;
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DbErr, DeleteResult, EntityTrait, ModelTrait, QueryFilter, TransactionTrait,
};
use uuid::Uuid;

use crate::{Database, entity};

impl Database {
    pub async fn get_users(&self) -> Result<Vec<entity::user::Model>, ApiError> {
        let users = entity::user::Entity::find().all(&self.conn).await?;

        Ok(users)
    }

    pub async fn get_user(&self, id: Uuid) -> Result<Option<entity::user::Model>, ApiError> {
        let user = entity::user::Entity::find()
            .filter(entity::user::Column::Id.eq(id))
            .one(&self.conn)
            .await?;

        if user.is_none() {
            return Err(ApiError::NotFound);
        }

        Ok(user)
    }

    pub async fn create_user(
        &self,
        name: String,
        username: String,
        password: String,
    ) -> Result<entity::user::Model, ApiError> {
        let argon2 = Argon2::default();

        let salt = SaltString::generate(&mut OsRng);
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|err| ApiError::Argon2Error(err.to_string()))?
            .to_string();

        let user = self
            .conn
            .transaction::<_, entity::user::Model, DbErr>(|txn| {
                Box::pin(async move {
                    let user = entity::user::ActiveModel {
                        id: NotSet,
                        name: Set(name),
                        username: Set(username),
                    };

                    let user: entity::user::Model = user.insert(txn).await?;

                    let local_auth = entity::local_auth::ActiveModel {
                        id: Set(user.id),
                        hash: Set(hash),
                        password_change_required: NotSet,
                    };

                    local_auth.insert(txn).await?;
                    Ok(user)
                })
            })
            .await?;
        Ok(user)
    }

    pub async fn verify_local_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Uuid, ApiError> {
        let user = entity::user::Entity::find()
            .filter(entity::user::Column::Username.eq(username))
            .one(&self.conn)
            .await?
            .ok_or(ApiError::Unauthorized)?;

        let local_auth = user
            .find_related(entity::local_auth::Entity)
            .one(&self.conn)
            .await?
            .ok_or(ApiError::Unauthorized)?;

        let argon2 = Argon2::default();

        let password_hash = PasswordHash::new(&local_auth.hash)
            .map_err(|err| ApiError::Argon2Error(err.to_string()))?;

        if let Err(_) = argon2.verify_password(password.as_bytes(), &password_hash) {
            return Err(ApiError::Unauthorized);
        }

        Ok(user.id)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<DeleteResult, ApiError> {
        let user = entity::user::Entity::delete_by_id(id)
            .exec(&self.conn)
            .await?;

        if user.rows_affected == 0 {
            return Err(ApiError::NotFound);
        }

        Ok(user)
    }

    pub async fn verify_ldap_user() {}

    pub async fn change_user_password() {}
}
