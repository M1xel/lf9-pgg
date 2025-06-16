pub mod controller;
pub mod db;
pub mod error;
pub mod utoipa;

pub use db::Database;
pub use db::entity;

use dotenvy;
use std::env;

#[cfg(not(test))]
fn get_env_var(name: &str) -> dotenvy::Result<String> {
    dotenvy::var(name)
}

#[cfg(test)]
fn get_env_var(name: &str) -> Result<String, std::env::VarError> {
    std::env::var(name)
}

// TODO: Extract build_database_url into a utils module or similar
pub fn build_database_url() -> String {
    let db_user = get_env_var("DB_USER").unwrap_or_else(|_| "pgg".to_owned());
    let db_name = get_env_var("DB_NAME").unwrap_or_else(|_| "pgg".to_owned());
    let db_password = get_env_var("DB_PASSWORD").unwrap_or_else(|_| "pgg".to_owned());
    let db_host = get_env_var("DB_HOST").expect("DB_HOST must be set in .env");
    let db_port = get_env_var("DB_PORT")
        .map(|x| x.parse::<u16>().expect("DB_PORT is not a valid port"))
        .unwrap_or(5432);

    let result = format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );

    println!("Database URL: {}", result);
    result
}
