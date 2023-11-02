use crate::{extractors::Claims, types::ErrorMessage};
use actix_web::{get, web, HttpResponse, Responder};
use basis::functions;
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

fn db_em(err: &str, desc: String) -> ErrorMessage {
    ErrorMessage {
        error: Some(err.to_string()),
        error_description: Some(desc.to_string()),
        message: "Internal Server Error".to_string(),
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
    let conn = pool.get();
    if let Err(e) = conn {
        return HttpResponse::NotImplemented().json(db_em("DB Connection Error", e.to_string()));
    }
    let mut conn = conn.unwrap();
    let mut daten = service::ServiceData::new(&mut conn, 1, "test");
    let list = service::client::get_user_list(&mut daten);
    //let list = service::reps::benutzer::get_all(&mut conn, 1);
    match list {
        Ok(list) => {
            let mut ben = Vec::new();
            for b in list {
                let mut bc = b.clone();
                bc.passwort = Some("xxx".to_string());
                ben.push(bc);
            }
            HttpResponse::Ok().json(ben)
        }
        Err(e) => HttpResponse::NotImplemented().json(db_em("DB Error", e.to_string())),
    }
}
