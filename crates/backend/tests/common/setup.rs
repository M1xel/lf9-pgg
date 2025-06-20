use backend::{Database, build_database_url};
use log::{debug, info};
use migration::{Migrator, MigratorTrait};
use testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner};
use testcontainers_modules::{postgres::Postgres, redis::Redis};

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

    debug!("PostgreSQL container started on port: {}", postgres_port);
    debug!("Redis container started on port: {}", redis_port);

    // Wait for PostgreSQL to be ready
    wait_for_postgres_ready(&postgres).await;

    unsafe {
        std::env::set_var("DB_HOST", "127.0.0.1");
        std::env::set_var("DB_PORT", postgres_port.to_string());
        std::env::set_var("DB_NAME", "test_db");
        std::env::set_var("DB_USER", "postgres");
        std::env::set_var("DB_PASSWORD", "postgres");
        std::env::set_var("REDIS_HOST", "127.0.0.1");
        std::env::set_var("REDIS_PORT", redis_port.to_string());
    }

    let database_url = build_database_url();
    info!("Database URL: {}", database_url);

    let database = Database::new(database_url.into()).await.unwrap();

    Migrator::up(database.connection(), None).await.unwrap();

    (postgres, redis, database)
}

async fn wait_for_postgres_ready(container: &ContainerAsync<Postgres>) {
    info!("Waiting for PostgreSQL to be ready...");

    for attempt in 1..=30 {
        match container.stdout_to_vec().await {
            Ok(logs) => {
                let log_string = String::from_utf8_lossy(&logs);

                if log_string.contains("database system is ready to accept connections") {
                    info!("PostgreSQL is ready after {} attempts", attempt);
                    return;
                }

                debug!("Attempt {}: PostgreSQL not ready yet", attempt);
            }
            Err(e) => {
                debug!("Attempt {}: Failed to read logs: {}", attempt, e);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    panic!("PostgreSQL failed to become ready within 30 seconds");
}
