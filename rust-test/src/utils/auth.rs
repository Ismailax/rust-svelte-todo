use actix_web::{HttpRequest, http::header};

pub fn extract_token(req: &HttpRequest) -> Option<String> {
    // Try Authorization: Bearer <token>
    if let Some(bearer) = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer"))
        .map(|s| s.trim().to_string())
    {
        return Some(bearer);
    }

    // Else try cookie
    req.cookie("access_token").map(|c| c.value().to_string())
}
