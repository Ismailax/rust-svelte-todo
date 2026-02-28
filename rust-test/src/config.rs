use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub frontend_url: String,

    // JWT
    pub jwt_private_key_path: String,
    pub jwt_public_key_path: String,
    pub jwt_issuer: String,
    pub jwt_access_ttl_secs: i64,

    // Cookies
    pub cookie_secure: bool,
    pub cookie_same_site: String,
    pub http_only: bool,
}

pub fn load_env() -> AppConfig {
    dotenv().ok();

    AppConfig {
        host: env::var("APP_HOST").expect("Missing APP_HOST"),
        port: env::var("APP_PORT")
            .expect("Missing APP_PORT")
            .parse()
            .expect("APP_PORT must be number"),

        frontend_url: env::var("FRONTEND_URL").expect("Missing FRONTEND_URL"),

        jwt_private_key_path: env::var("JWT_PRIVATE_KEY_PATH")
            .expect("Missing JWT_PRIVATE_KEY_PATH"),
        jwt_public_key_path: env::var("JWT_PUBLIC_KEY_PATH").expect("Missing JWT_PUBLIC_KEY_PATH"),
        jwt_issuer: env::var("JWT_ISSUER").unwrap_or_else(|_| "app".into()),
        jwt_access_ttl_secs: env::var("JWT_ACCESS_TTL_SECS")
            .unwrap_or_else(|_| "3600".into())
            .parse()
            .expect("JWT_ACCESS_TTL_SECS must be number"),

        cookie_secure: env::var("COOKIE_SECURE")
            .unwrap_or_else(|_| "false".into())
            .parse()
            .unwrap_or(false),

        cookie_same_site: env::var("COOKIE_SAME_SITE").unwrap_or_else(|_| "Lax".into()),

        http_only: env::var("HTTP_ONLY")
            .unwrap_or_else(|_| "true".into())
            .parse()
            .unwrap_or(true),
    }
}
