use sea_orm::{ConnectOptions, DatabaseConnection};

pub mod entity;
mod group;
pub mod project;
mod user;

#[derive(Clone)]
pub struct Database {
    conn: DatabaseConnection,
}

impl Database {
    pub async fn new(options: ConnectOptions) -> Result<Self, sea_orm::DbErr> {
        Ok(Database {
            conn: sea_orm::Database::connect(options).await?,
        })
    }

    pub fn connection(&self) -> &DatabaseConnection {
        &self.conn
    }
}
