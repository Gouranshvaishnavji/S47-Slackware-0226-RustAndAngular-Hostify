pub fn get_port() -> u16 {
    std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080)
}

pub fn get_auth_token() -> String {
    std::env::var("AUTH_TOKEN").unwrap_or_else(|_| "devtoken123".to_string())
}

pub fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret".to_string())
}

pub fn get_admin_user() -> String {
    std::env::var("ADMIN_USER").unwrap_or_else(|_| "admin".to_string())
}

pub fn get_admin_password() -> String {
    std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "password".to_string())
}
