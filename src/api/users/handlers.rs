use crate::{
    base::{BpError, DbPool, UndoPool},
    extractors::Claims,
    types::ErrorMessage,
};
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder, Result};
use basis::functions;
use rep::models::Benutzer;
use std::{collections::HashSet, sync::Mutex};

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

#[get("/")]
async fn index() -> Result<&'static str, BpError> {
    functions::mach_nichts();
    Err(BpError::InternalError)
}

use lazy_static::lazy_static;
use std::sync::atomic::{AtomicUsize, Ordering};

lazy_static! {
    static ref SESSION_ID: AtomicUsize = AtomicUsize::new(1);
}

fn get_session_id(session: &Session) -> Result<usize, BpError> {
    if let Some(id) = session
        .get::<usize>("sid")
        .map_err(|e| BpError::from(e.to_string()))?
    {
        Ok(id)
    } else {
        let id = SESSION_ID.fetch_add(1, Ordering::SeqCst);
        session
            .insert("sid", id)
            .map_err(|e| BpError::from(e.to_string()))?;
        Ok(id)
    }
}

#[get("")]
pub async fn list(
    session: Session,
    pool: web::Data<DbPool>,
    undo: web::Data<Mutex<UndoPool>>,
) -> Result<impl Responder, BpError> {
    let session_id = get_session_id(&session)?;
    // access the session state
    // if let Some(count) = session
    //     .get::<i32>("counter")
    //     .map_err(|e| BpError::from(e.to_string()))?
    // {
    //     println!("SESSION value: {}", count);
    //     // modify the session state
    //     session
    //         .insert("counter", count + 1)
    //         .map_err(|e| BpError::from(e.to_string()))?;
    // } else {
    //     session
    //         .insert("counter", 1)
    //         .map_err(|e| BpError::from(e.to_string()))?;
    // }
    let mut ul = undo.lock().map_err(|e| BpError::from(e.to_string()))?;
    let _us = (*ul)
        .map
        .entry(session_id)
        .or_insert(service::UndoRedoStack::new());
    let list = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get()?; //.expect("couldn't get db connection from pool");
        let mut data = service::ServiceData::new(1, "test");
        let list = service::client::get_user_list(&mut conn, &mut data)?;
        Ok(list) as Result<Vec<Benutzer>, BpError>
    })
    .await??;
    // let list = web::block(move || {
    //     // Obtaining a connection from the pool is also a potentially blocking operation.
    //     // So, it should be called within the `web::block` closure, as well.
    //     let mut conn = pool.get()?; //.expect("couldn't get db connection from pool");
    //     let tr = conn.transaction::<Vec<Benutzer>, BpError, _>(|e| {
    //         let mut daten = service::ServiceData::new(e, 1, "test");
    //         let list = service::client::get_user_list(&mut daten)?;
    //         Ok(list)
    //     });
    //     tr
    // })
    // .await??;
    // Masks passwords.
    let mut ben = Vec::new();
    for b in list {
        let mut bc = b.clone();
        bc.passwort = Some("xxx".to_string());
        ben.push(bc);
    }
    Ok(HttpResponse::Ok().json(ben))
}
