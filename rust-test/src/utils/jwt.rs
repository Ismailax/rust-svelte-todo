use actix_web::cookie::{Cookie, SameSite, time::Duration as CookieDuration};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::config::AppConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub iss: String,
}

fn same_site_from(cfg: &AppConfig) -> SameSite {
    match cfg.cookie_same_site.as_str() {
        "Strict" => SameSite::Strict,
        "None" => SameSite::None,
        _ => SameSite::Lax,
    }
}

fn load_private_key_pem(cfg: &AppConfig) -> Result<Vec<u8>, String> {
    Ok(cfg.jwt_private_key_pem.as_bytes().to_vec())
}

fn load_public_key_pem(cfg: &AppConfig) -> Result<Vec<u8>, String> {
    Ok(cfg.jwt_public_key_pem.as_bytes().to_vec())
}

pub fn create_access_token(user_id: i32, cfg: &AppConfig) -> Result<String, String> {
    let now = OffsetDateTime::now_utc();
    let exp = now + Duration::seconds(cfg.jwt_access_ttl_secs);

    let claims = Claims {
        sub: user_id.to_string(),
        iat: now.unix_timestamp(),
        exp: exp.unix_timestamp(),
        iss: cfg.jwt_issuer.clone(),
    };

    let private_pem = load_private_key_pem(cfg)?;
    let key = EncodingKey::from_ec_pem(&private_pem).map_err(|_| "bad_private_key".to_string())?;
    let header = Header::new(Algorithm::ES256);

    encode(&header, &claims, &key).map_err(|_| "jwt_encode_error".to_string())
}

pub fn verify_token(token: &str, cfg: &AppConfig) -> Result<Claims, String> {
    let public_pem = load_public_key_pem(cfg)?;
    let key = DecodingKey::from_ec_pem(&public_pem).map_err(|_| "bad_public_key".to_string())?;

    let mut v = Validation::new(Algorithm::ES256);
    v.set_required_spec_claims(&["exp", "iat", "iss", "sub"]);
    v.validate_exp = true;
    v.set_issuer(&[cfg.jwt_issuer.clone()]);

    decode::<Claims>(token, &key, &v)
        .map(|td| td.claims)
        .map_err(|_| "jwt_invalid".to_string())
}

pub fn build_auth_cookie(token: &str, cfg: &AppConfig) -> Cookie<'static> {
    let mut c = Cookie::build("access_token", token.to_string())
        .http_only(cfg.http_only)
        .same_site(same_site_from(cfg))
        .path("/")
        .finish();

    c.set_max_age(CookieDuration::seconds(cfg.jwt_access_ttl_secs));
    c.set_secure(cfg.cookie_secure);
    c
}

pub fn clear_auth_cookie(cfg: &AppConfig) -> Cookie<'static> {
    Cookie::build("access_token", "")
        .path("/")
        .http_only(cfg.http_only)
        .same_site(same_site_from(cfg))
        .max_age(CookieDuration::seconds(0))
        .finish()
}
