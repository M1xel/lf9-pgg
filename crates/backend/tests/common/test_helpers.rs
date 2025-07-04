use backend::Database;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicU64, Ordering};
use testcontainers::ContainerAsync;
use testcontainers_modules::{postgres::Postgres, redis::Redis};

use super::setup;

struct TestState {
    _postgres: ContainerAsync<Postgres>,
    _redis: ContainerAsync<Redis>,
}

lazy_static! {
    static ref TEST_STATE: tokio::sync::OnceCell<TestState> = tokio::sync::OnceCell::new();
}

pub async fn get_database() -> Database {
    let _state = TEST_STATE
        .get_or_init(|| async {
            let (postgres, redis, _database) = setup().await;
            TestState {
                _postgres: postgres,
                _redis: redis,
            }
        })
        .await;

    // Create a new database connection for each test
    let database_url = backend::build_database_url();
    let mut opts = sea_orm::ConnectOptions::new(database_url);
    opts.max_connections(5)
        .min_connections(1)
        .connect_timeout(std::time::Duration::from_secs(10))
        .acquire_timeout(std::time::Duration::from_secs(10));
    
    Database::new(opts).await.unwrap()
}

static TEST_COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn get_unique_test_id() -> String {
    let counter = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("test_{}_{}", counter, timestamp)
}

#[derive(Clone)]
pub struct TestContext {
    pub test_id: String,
    pub created_users: std::sync::Arc<std::sync::Mutex<Vec<uuid::Uuid>>>,
    pub created_projects: std::sync::Arc<std::sync::Mutex<Vec<uuid::Uuid>>>,
}

impl TestContext {
    pub fn new() -> Self {
        Self {
            test_id: get_unique_test_id(),
            created_users: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            created_projects: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub async fn cleanup_all(&self, db: &Database) {
        self.cleanup_projects(db).await;
        self.cleanup_users(db).await;
    }
}

#[macro_export]
macro_rules! create_test_app {
    () => {{
        let db = $crate::common::test_helpers::get_database().await;

        actix_web::test::init_service(
            actix_web::App::new()
                .app_data(actix_web::web::Data::new(db))
                .service(
                    actix_web::web::scope("/api/v1")
                        .configure(backend::controller::register_controllers),
                ),
        )
        .await
    }};
}
