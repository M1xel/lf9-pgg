use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::cookie::SameSite;
use actix_web::{App, HttpServer, cookie::Key, middleware::Logger, web};
use log::debug;
use utoipa_swagger_ui::SwaggerUi;

mod controller;
mod db;
mod error;
mod utils;
mod utoipa;

use db::Database;
use log::info;
use migration::Migrator;
use migration::MigratorTrait;
use utils::{build_database_url, get_env_var};

#[derive(Clone)]
struct AppConfig {
    ldap_auth: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = build_database_url();

    let database = Database::new(database_url.into()).await.unwrap();

    info!("Running migrations");
    Migrator::up(database.connection(), None).await.unwrap();
    info!("Migrations completed");

    let redis_conn = connect_to_redis_database().await;

    let app_config = AppConfig { ldap_auth: false };

    // use dotenvy here to get SECRET_KEY
    let secret_key = Key::generate();
    debug!("Secret Key {:?}", secret_key.master());

    HttpServer::new(move || {
        let session_middleware = SessionMiddleware::builder(redis_conn.clone(), secret_key.clone());

        let session_middleware = if cfg!(debug_assertions) {
            session_middleware.cookie_secure(false)
        } else {
            session_middleware
                .cookie_same_site(SameSite::Strict)
                .cookie_secure(true)
        };

        let session_middleware = session_middleware.build();

        let app = App::new()
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(app_config.clone()))
            .wrap(Logger::default())
            .wrap(session_middleware)
            .service(web::scope("/api/v1").configure(controller::register_controllers))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/api-docs/openapi.json",
                crate::utoipa::ApiDoc::openapi_spec(),
            ));

        #[cfg(feature = "serve")]
        let app = {
            info!("running serve");
            app.default_service(
                web::get().to(async || NamedFile::open_async("./web/index.html").await),
            )
        };

        app
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

pub async fn connect_to_redis_database() -> RedisSessionStore {
    let redis_host = get_env_var("REDIS_HOST").expect("REDIS_HOST must be set in .env");
    let redis_port = get_env_var("REDIS_PORT")
        .map(|x| x.parse::<u16>().expect("REDIS_PORT is not a valid port"))
        .unwrap_or(6379);
    let redis_connection_string = format!("redis://{}:{}", redis_host, redis_port);

    RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use temp_env::{with_var, with_vars};

    #[test]
    #[serial]
    fn build_database_url_with_defaults() {
        with_vars([("DB_HOST", Some("localhost"))], || {
            let expected_url = "postgresql://pgg:pgg@localhost:5432/pgg";
            let actual_url = build_database_url();
            assert_eq!(
                actual_url, expected_url,
                "Database URL should use default values for unset env vars."
            );
        });
    }

    #[test]
    #[serial]
    fn build_database_url_with_all_vars() {
        with_vars(
            [
                ("DB_USER", Some("testuser")),
                ("DB_NAME", Some("testdb")),
                ("DB_PASSWORD", Some("testpass")),
                ("DB_HOST", Some("otherhost.internal")),
                ("DB_PORT", Some("5433")),
            ],
            || {
                let expected_url = "postgresql://testuser:testpass@otherhost.internal:5433/testdb";
                let actual_url = build_database_url();
                assert_eq!(
                    actual_url, expected_url,
                    "Database URL should use all provided env vars."
                );
            },
        );
    }

    #[test]
    #[serial]
    #[should_panic(expected = "DB_HOST must be set in .env")]
    fn build_database_url_missing_host_panics() {
        with_var("DB_HOST", None::<&str>, || {
            build_database_url();
        });
    }

    #[test]
    #[serial]
    fn connect_to_redis_database_with_defaults() {
        with_vars([("REDIS_HOST", Some("localhost"))], || {
            let expected_conn_string = "redis://localhost:6379";

            let redis_host = get_env_var("REDIS_HOST").unwrap_or_default();
            let redis_port = get_env_var("REDIS_PORT")
                .map(|x| x.parse::<u16>().unwrap_or(6379))
                .unwrap_or(6379);
            let actual_conn_string = format!("redis://{redis_host}:{redis_port}");

            assert_eq!(
                actual_conn_string, expected_conn_string,
                "Redis connection string should use default port when not specified."
            );
        });
    }

    #[test]
    #[serial]
    fn connect_to_redis_database_with_custom_port() {
        with_vars(
            [
                ("REDIS_HOST", Some("redis.internal")),
                ("REDIS_PORT", Some("6380")),
            ],
            || {
                let expected_conn_string = "redis://redis.internal:6380";

                let redis_host = get_env_var("REDIS_HOST").unwrap_or_default();
                let redis_port = get_env_var("REDIS_PORT")
                    .map(|x| x.parse::<u16>().unwrap_or(6379))
                    .unwrap_or(6379);
                let actual_conn_string = format!("redis://{}:{}", redis_host, redis_port);

                assert_eq!(
                    actual_conn_string, expected_conn_string,
                    "Redis connection string should use specified host and port."
                );
            },
        );
    }

    #[test]
    #[serial]
    fn check_if_no_env_variables_are_loaded_from_environment_file() {
        assert_eq!(env::var("DB_NAME"), Err(env::VarError::NotPresent));
        assert_eq!(env::var("DB_USER"), Err(env::VarError::NotPresent));
        assert_eq!(env::var("DB_PASSWORD"), Err(env::VarError::NotPresent));
        assert_eq!(env::var("DB_HOST"), Err(env::VarError::NotPresent));
        assert_eq!(env::var("DB_PORT"), Err(env::VarError::NotPresent));

        assert_eq!(env::var("REDIS_PORT"), Err(env::VarError::NotPresent));
        assert_eq!(env::var("REDIS_HOST"), Err(env::VarError::NotPresent));
    }
}
