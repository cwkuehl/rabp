use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/api")
        .service(super::messages::routes())
        .service(super::users::routes())
}
