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

            let resp = test::TestRequest::get()
                .uri("/api/v1/ok")
                .send_request(&app)
                .await;

            let resp_status = resp.status();

            let resp_body = test::read_body(resp).await;
            let resp_body_str = String::from_utf8_lossy(&resp_body);
            assert!(
                resp_body_str.contains("available"),
                "Expected 'available' in response body"
            );

            assert!(resp_status.is_success(), "Expected success response");
        })
        .await;
    }
}
