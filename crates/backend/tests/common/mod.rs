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

    println!("PostgreSQL container started on port: {}", postgres_port);
    println!("Redis container started on port: {}", redis_port);

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
    println!("Database URL: {}", database_url);

    let database = Database::new(database_url.into()).await.unwrap();

    Migrator::up(database.connection(), None).await.unwrap();

    (postgres, redis, database)
}

async fn wait_for_postgres_ready(container: &ContainerAsync<Postgres>) {
    use sea_orm::{Database as SeaOrmDatabase, DbErr};

    println!("Waiting for PostgreSQL to be ready...");

    let postgres_port = container.get_host_port_ipv4(5432).await.unwrap();
    let connection_string = format!(
        "postgresql://postgres:postgres@127.0.0.1:{}/test_db",
        postgres_port
    );

    for attempt in 1..=30 {
        match SeaOrmDatabase::connect(&connection_string).await {
            Ok(conn) => match conn.ping().await {
                Ok(_) => {
                    println!("PostgreSQL is ready after {} attempts", attempt);
                    return;
                }
                Err(_) => {
                    println!("Attempt {}: PostgreSQL connection failed ping", attempt);
                }
            },
            Err(DbErr::Conn(_)) => {
                println!("Attempt {}: PostgreSQL connection refused", attempt);
            }
            Err(_) => {
                println!("Attempt {}: PostgreSQL other error", attempt);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    panic!("PostgreSQL failed to become ready within 30 seconds");
}
