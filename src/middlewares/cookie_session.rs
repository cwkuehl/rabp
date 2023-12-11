use actix_session::{
    config::{BrowserSession, CookieContentSecurity},
    storage::CookieSessionStore,
    SessionMiddleware,
};
use actix_web::cookie::{Key, SameSite};

pub fn cookie_session() -> SessionMiddleware<CookieSessionStore> {
    let secret_key = Key::generate();
    SessionMiddleware::builder(CookieSessionStore::default(), secret_key)
        .cookie_name(String::from("rabp-cookie"))
        .cookie_secure(true)
        .session_lifecycle(BrowserSession::default())
        .cookie_same_site(SameSite::Strict)
        .cookie_content_security(CookieContentSecurity::Private)
        .cookie_http_only(true)
        .build()
}
