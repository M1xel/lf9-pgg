use ldap3::{LdapConn, Scope, SearchEntry};

/// Authenticates a user against an LDAP server.
///
/// # Arguments
/// * `ldap_server` - The LDAP server URL.
/// * `base_dn` - The base DN for the LDAP directory.
/// * `username` - The username to authenticate.
/// * `password` - The password for the user.
///
/// # Returns
/// * `Ok(true)` if authentication is successful.
/// * `Ok(false)` if authentication fails.
/// * `Err` if an error occurs during the process.
pub fn authenticate_user(
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

