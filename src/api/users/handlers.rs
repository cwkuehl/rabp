use crate::{base::BpError, extractors::Claims, types::ErrorMessage};
use actix_web::{get, web, HttpResponse, Responder, Result};
use basis::functions;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::sqlite::SqliteConnection;
use rep::models::Benutzer;
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

// fn db_em(err: &str, desc: String) -> ErrorMessage {
//     ErrorMessage {
//         error: Some(err.to_string()),
//         error_description: Some(desc.to_string()),
//         message: "Internal Server Error".to_string(),
//     }
// }

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

// #[get("")]
// pub async fn list0(pool: web::Data<DbPool>) -> impl Responder {
//     functions::mach_nichts();
//     let conn = pool.get();
//     if let Err(e) = conn {
//         return HttpResponse::NotImplemented().json(db_em("DB Connection Error", e.to_string()));
//     }
//     let mut conn = conn.unwrap();
//     let mut daten = service::ServiceData::new(&mut conn, 1, "test");
//     let list = service::client::get_user_list(&mut daten);
//     match list {
//         Ok(list) => {
//             let mut ben = Vec::new();
//             for b in list {
//                 let mut bc = b.clone();
//                 bc.passwort = Some("xxx".to_string());
//                 ben.push(bc);
//             }
//             HttpResponse::Ok().json(ben)
//         }
//         Err(e) => HttpResponse::NotImplemented().json(db_em("DB Error", e.to_string())),
//     }
// }

// #[get("")]
// pub async fn list(pool: web::Data<DbPool>) -> Result<impl Responder> {
//     functions::mach_nichts();
//     let list = web::block(move || {
//         // Obtaining a connection from the pool is also a potentially blocking operation.
//         // So, it should be called within the `web::block` closure, as well.
//         let mut conn = pool.get()
//           .map_err(|e| service::ServiceError::error_string(&e.to_string()))?; //.expect("couldn't get db connection from pool");
//         let mut daten = service::ServiceData::new(&mut conn, 1, "test");
//         let list = service::client::get_user_list(&mut daten);
//         //list
//         let err: Result<Vec<Benutzer>, service::ServiceError> =
//             Err(service::ServiceError::DieselError {
//                 source: diesel::result::Error::NotInTransaction,
//             });
//         err
//         //insert_new_user(&mut conn, name)
//     })
//     .await?
//     .map_err(error::ErrorFailedDependency)?;
//     // let conn = pool.get();
//     // if let Err(e) = conn {
//     //     return Ok(HttpResponse::NotImplemented().json(db_em("DB Connection Error", e.to_string())));
//     // }
//     // let mut conn = conn.unwrap();
//     // let mut daten = service::ServiceData::new(&mut conn, 1, "test");
//     // let list = service::client::get_user_list(&mut daten);
//     //let list = service::reps::benutzer::get_all(&mut conn, 1);
//     // Masks passwords.
//     let mut ben = Vec::new();
//     for b in list {
//         let mut bc = b.clone();
//         bc.passwort = Some("xxx".to_string());
//         ben.push(bc);
//     }
//     //Err::<impl Responder, actix_web::Error>(error::ErrorNotImplemented("Doch OK"))
//     Ok(HttpResponse::Ok().json(ben))
// }

#[get("/")]
async fn index() -> Result<&'static str, BpError> {
    Err(BpError::InternalError)
}

#[get("")]
pub async fn list(pool: web::Data<DbPool>) -> Result<impl Responder, BpError> {
    functions::mach_nichts();
    let list = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get()?; //.expect("couldn't get db connection from pool");
        let mut daten = service::ServiceData::new(&mut conn, 1, "test");
        let list = service::client::get_user_list(&mut daten)?;
        Ok(list) as Result<Vec<Benutzer>, BpError>
    })
    .await??;
    // Masks passwords.
    let mut ben = Vec::new();
    for b in list {
        let mut bc = b.clone();
        bc.passwort = Some("xxx".to_string());
        ben.push(bc);
    }
    Ok(HttpResponse::Ok().json(ben))
}
