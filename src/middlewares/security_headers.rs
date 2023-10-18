use actix_web::{http::header, middleware::DefaultHeaders};

pub fn security_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        .add((header::X_XSS_PROTECTION.as_str(), "0"))
        .add((
            header::STRICT_TRANSPORT_SECURITY.as_str(),
            "max-age=31536000; includeSubDomains",
        ))
        .add((header::X_FRAME_OPTIONS.as_str(), "deny"))
        .add((header::X_CONTENT_TYPE_OPTIONS.as_str(), "nosniff"))
        .add((
            header::CONTENT_SECURITY_POLICY.as_str(),
            "default-src 'self'; frame-ancestors 'none';",
        ))
        .add((
            header::CACHE_CONTROL.as_str(),
            "no-cache, no-store, max-age=0, must-revalidate",
        ))
        .add((header::PRAGMA.as_str(), "no-cache"))
        .add((header::EXPIRES.as_str(), "0"))
}
