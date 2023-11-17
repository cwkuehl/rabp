use super::handlers;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/users")
        .service(handlers::index)
        .service(handlers::list)
        .service(handlers::listu)
        .service(handlers::listr)
}
