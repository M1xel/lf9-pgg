use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use ldap3::{LdapConn, Scope, SearchEntry};
use serde::Deserialize;
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

// Function to authenticate user against LDAP server
fn authenticate_user(
    ldap_server: &str,
    base_dn: &str,
    username: &str,
    password: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Establish connection to LDAP server
    let ldap = LdapConn::new(ldap_server)?;

    // Search for the user in the LDAP directory
    let (rs, _res) = ldap
        .search(
            &format!("ou=users,{}", base_dn), // Search under "ou=users"
            Scope::Subtree,                   // Search all levels
            &format!("(uid={})", username),   // Filter by username
            vec!["dn"],                       // Retrieve the distinguished name (DN)
        )?
        .success()?;

    // If user is found, attempt to authenticate with their DN and password
    if let Some(entry) = rs.into_iter().next() {
        let user_dn = SearchEntry::construct(entry).dn; // Extract user DN

        // Reconnect and bind with user credentials
        let user_ldap = LdapConn::new(ldap_server)?;
        let auth_result = user_ldap.simple_bind(&user_dn, password)?.success();
        return Ok(auth_result.is_ok()); // Return true if authentication succeeds
    }

    Ok(false) // Return false if user is not found
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
