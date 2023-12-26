use super::handlers;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/diary")
        .service(handlers::last)
        .service(handlers::list)
}
