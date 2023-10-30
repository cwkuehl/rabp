use crate::{base::functions, extractors::Claims, types::ErrorMessage};
use actix_web::{get, web, HttpResponse, Responder};
//use r2d2::Pool;
//use r2d2_sqlite::SqliteConnectionManager;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::sqlite::SqliteConnection;
use std::collections::HashSet;

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

#[get("/xxx")]
pub async fn listxxx(claims: Claims, _pool: web::Data<DbPool>) -> impl Responder {
    functions::mach_nichts();
    if claims.validate_permissions(&HashSet::from(["read:admin-messages".to_string()])) {
        HttpResponse::Ok().json(ErrorMessage {
            error: None,
            error_description: None,
            message: "Users OK".to_string(),
        })
    } else {
        // HttpResponse::InternalServerError().into()
        HttpResponse::Ok().json(ErrorMessage {
            error: Some("insufficient_permissions".to_string()),
            error_description: Some("Requires read:admin-messages".to_string()),
            message: "Permission denied".to_string(),
        })
    }
}

/*
apigee concept:
GET (read), DELETE (delete), POST (create), PUT or PATCH (update)
in URLs, nouns are good; verbs are bad
resource created: 201 Created, response header Content-Location: /users/1
get response: 200 OK
wrong method: 405 Method Not Allowed, with response header Allow
401 Unauthorized: response header WWW-Authenticate: Bearer
json response with error: 400 Bad Request, with response body: {"error":"insufficient_permissions","error_description":"Requires read:admin-messages","message":"Permission denied"} or developerMessage, userMessage, errorCode
Versioning: update with PATCH instead of PUT
*/

#[get("")]
pub async fn list(pool: web::Data<DbPool>) -> impl Responder {
    functions::mach_nichts();
    let mut conn = pool.get().unwrap();
    let list = crate::reps::benutzer::get_all(&mut conn, 1);
    match list {
      Ok(list) => HttpResponse::Ok().json(list),
      Err(e) => HttpResponse::NotImplemented().json(ErrorMessage {
          error: None,
          error_description: None,
          message: e.to_string(),
      }),
    }
    // if let Ok(list) = list {
    //     return HttpResponse::Ok().json(list);
    // } else {
    //     return HttpResponse::NotImplemented().json(ErrorMessage {
    //         error: None,
    //         error_description: None,
    //         message: "DB Error".to_string(),
    //     });
    // }

    //let mut stmt = conn.prepare("SELECT * FROM Benutzer").unwrap();
    //let rows = stmt.query([]).unwrap();
    // HttpResponse::ImATeapot().json(ErrorMessage {
    //     error: None,
    //     error_description: None,
    //     message: "Users OK".to_string(),
    // })
}
