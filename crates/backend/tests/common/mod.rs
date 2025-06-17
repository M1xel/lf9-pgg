use backend::{Database, build_database_url};
use migration::{Migrator, MigratorTrait};
use testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner};
use testcontainers_modules::{postgres::Postgres, redis::Redis};

pub mod test_helpers;

pub async fn setup() -> (ContainerAsync<Postgres>, ContainerAsync<Redis>, Database) {
    let postgres = Postgres::default()
        .with_env_var("POSTGRES_DB", "test_db")
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .start()
        .await
        .expect("Failed to start PostgreSQL container");

    let redis = Redis::default()
        .start()
        .await
        .expect("Failed to start Redis container");

    let postgres_port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let redis_port = redis.get_host_port_ipv4(6379).await.unwrap();

    unsafe {
        std::env::set_var("DB_HOST", "127.0.0.1");
        std::env::set_var("DB_PORT", postgres_port.to_string());
        std::env::set_var("DB_NAME", "test_db");
        std::env::set_var("DB_USER", "postgres");
        std::env::set_var("DB_PASSWORD", "postgres");
        std::env::set_var("REDIS_HOST", "127.0.0.1");
        std::env::set_var("REDIS_PORT", redis_port.to_string());
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let database_url = build_database_url();
    let database = Database::new(database_url.into()).await.unwrap();

    Migrator::up(database.connection(), None).await.unwrap();

    (postgres, redis, database)
}
