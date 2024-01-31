use actix_web::{web, Scope};
use basis::functions;

pub fn routes() -> Scope {
    let mut r = web::scope("/api")
        .service(super::version)
        .service(super::health)
        .service(super::diary::routes());
    if functions::mach_nichts() != 0 {
        // API is not used.
        r = r
            .service(super::messages::routes())
            .service(super::users::routes());
    }
    r
}
