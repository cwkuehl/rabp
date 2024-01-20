use actix_web::{web, Scope};
use basis::functions;

pub fn routes() -> Scope {
    let mut r = web::scope("/api")
        .service(super::diary::routes())
        .service(super::messages::routes());
    if functions::mach_nichts() != 0 {
        // Users API is not used.
        r = r.service(super::users::routes())
    }
    r
}
