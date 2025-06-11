use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use ldap::authenticate_user; // Import the library function
use std::env;

// Struct to deserialize login request payload
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

// HTTP POST endpoint for user login
#[post("/login")]
async fn login(credentials: web::Json<LoginRequest>) -> impl Responder {
    // Get LDAP server and base DN from environment variables or use defaults
    let ldap_server = env::var("LDAP_SERVER").unwrap_or("ldap://127.0.0.1:389".to_string());
    let base_dn = env::var("LDAP_BASE_DN").unwrap_or("dc=schule,dc=local".to_string());

    // Authenticate user and return appropriate response
    match authenticate_user(&ldap_server, &base_dn, &credentials.username, &credentials.password) {
        Ok(true) => HttpResponse::Ok().body("Login erfolgreich"), // Login successful
        _ => HttpResponse::Unauthorized().body("Login fehlgeschlagen"), // Login failed
    }
}

// Main function to start the Actix Web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Initialize logger

    // Start HTTP server and bind to localhost:8080
    HttpServer::new(|| App::new().service(login))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
