use backend::Database;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicU64, Ordering};
use testcontainers::ContainerAsync;
use testcontainers_modules::{postgres::Postgres, redis::Redis};

use super::setup;

struct TestState {
    _postgres: ContainerAsync<Postgres>,
    _redis: ContainerAsync<Redis>,
    database: Database,
}

lazy_static! {
    static ref TEST_STATE: tokio::sync::OnceCell<TestState> = tokio::sync::OnceCell::new();
}

pub async fn get_database() -> &'static Database {
    let state = TEST_STATE
        .get_or_init(|| async {
            let (postgres, redis, database) = setup().await;
            TestState {
                _postgres: postgres,
                _redis: redis,
                database,
            }
        })
        .await;

    &state.database
}

static TEST_COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn get_unique_test_id() -> String {
    let counter = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("test_{}_{}", timestamp, counter)
}

pub struct UserFactory;

impl UserFactory {
    pub fn create_request(username: Option<String>, name: Option<String>) -> serde_json::Value {
        let test_id = get_unique_test_id();
        serde_json::json!({
            "username": username.unwrap_or_else(|| format!("user_{}", test_id)),
            "name": name.unwrap_or_else(|| format!("Test User {}", test_id)),
            "password": "password123"
        })
    }

    pub fn create_unique_request() -> serde_json::Value {
        Self::create_request(None, None)
    }
}

pub struct TestContext {
    pub test_id: String,
}

impl TestContext {
    pub fn new() -> Self {
        Self {
            test_id: get_unique_test_id(),
        }
    }

    pub fn create_user_data(&self, username_prefix: Option<&str>, name: Option<&str>) -> serde_json::Value {
        let username = username_prefix
            .map(|prefix| format!("{}_{}", prefix, self.test_id))
            .unwrap_or_else(|| format!("user_{}", self.test_id));
        
        UserFactory::create_request(Some(username), name.map(String::from))
    }
}

#[macro_export]
macro_rules! create_test_app {
    () => {{
        let db = $crate::common::test_helpers::get_database().await;

        actix_web::test::init_service(
            actix_web::App::new()
                .app_data(actix_web::web::Data::new(db.clone()))
                .service(
                    actix_web::web::scope("/api/v1")
                        .configure(backend::controller::register_controllers),
                ),
        )
        .await
    }};
}
