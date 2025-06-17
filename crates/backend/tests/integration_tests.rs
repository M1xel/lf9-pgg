mod common;
mod endpoints;

#[cfg(test)]
mod tests {
    use actix_web::{App, test, web};
    use backend::controller;

    use crate::common::test_helpers::{get_database, with_transaction};

    #[actix_web::test]
    async fn test_auth_with_transaction() {
        with_transaction(|_tx| async {
            let db = get_database().await;

            let app = test::init_service(
                App::new()
                    .app_data(web::Data::new(db.clone()))
                    .service(web::scope("/api/v1").configure(controller::register_controllers)),
            )
            .await;

            let req = test::TestRequest::get().uri("/api/v1/user").to_request();
            let resp = test::call_service(&app, req).await;

            assert!(resp.status().is_success() || resp.status().is_client_error());
        })
        .await;
    }
}
