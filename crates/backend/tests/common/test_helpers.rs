use backend::Database;
use lazy_static::lazy_static;
use sea_orm::{DatabaseTransaction, TransactionTrait};
use std::future::Future;
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

pub async fn with_transaction<F, Fut, R>(test: F) -> R
where
    F: FnOnce(DatabaseTransaction) -> Fut,
    Fut: Future<Output = R>,
{
    let db = get_database().await;
    let tx = db
        .connection()
        .begin()
        .await
        .expect("Failed to start transaction");
    test(tx).await
}
