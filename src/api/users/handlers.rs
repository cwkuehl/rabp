use crate::{
    api::{get_service_data, session_undo},
    base::{BpError, DbPool, UndoPool},
    extractors::Claims,
    types::ErrorMessage,
};
use actix_web::{get, web, HttpResponse, Responder, Result};
use basis::functions;
use rep::models::Benutzer;
use service::UndoList;
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
use std::sync::atomic::AtomicUsize;

lazy_static! {
    static ref SESSION_ID: AtomicUsize = AtomicUsize::new(1);
}

// #[get("/login/{clientid}/{userid}")]
// pub async fn login(
//     path: web::Path<(i32, String)>,
//     claims: Claims,
//     session: Session,
//     pool: web::Data<DbPool>,
//     undo: web::Data<Mutex<UndoPool>>,
// ) -> Result<impl Responder, BpError> {
//     let (clientid, userid) = path.into_inner();
//     let mut data = init_session(Some(claims), session.clone(), clientid, &userid)?;
//     let session_id = data.get_session_id();
//     // let mut data = service::ServiceData::new(clientid, &userid);
//     let f = move || {
//         // Obtaining a connection from the pool is also a potentially blocking operation.
//         // So, it should be called within the `web::block` closure, as well.
//         let mut con = pool.get()?;
//         let ulist = service::client::get_user_list(&mut con, &mut data, true, true)?;
//         Ok((ulist, data.ul)) as Result<(Vec<Benutzer>, UndoList), BpError>
//     };
//     let ulist = session_undo(session_id, undo, f).await?;
//     Ok(HttpResponse::Ok().json(ulist))
// }

#[get("")]
pub async fn list(
    claims: Claims,
    pool: web::Data<DbPool>,
    undo: web::Data<Mutex<UndoPool>>,
) -> Result<impl Responder, BpError> {
    let mut data = get_service_data(Some(claims), true)?;
    let session_id = data.get_session_id();
    let f = move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut con = pool.get()?;
        let ulist = service::client::get_user_list(&mut con, &mut data, true, true)?;
        Ok((ulist, data.ul)) as Result<(Vec<Benutzer>, UndoList), BpError>
    };
    let ulist = session_undo(session_id, undo, f).await?;
    Ok(HttpResponse::Ok().json(ulist))
}

#[get("/wo")]
pub async fn listwo(pool: web::Data<DbPool>) -> Result<impl Responder, BpError> {
    let mut data = service::ServiceData::new(1, "user");
    let (ulist, _ul2) = web::block(move || {
        let mut con = pool.get()?;
        service::client::undo(&mut con, &mut data)?; //, undolist)?;
        let ulist = service::client::get_user_list(&mut con, &mut data, true, false)?;
        Ok((ulist, data.ul)) as Result<(Vec<Benutzer>, UndoList), BpError>
    })
    .await??;
    Ok(HttpResponse::Ok().json(ulist))
}

#[get("/u")]
pub async fn listu(
    claims: Claims,
    pool: web::Data<DbPool>,
    undo: web::Data<Mutex<UndoPool>>,
) -> Result<impl Responder, BpError> {
    let mut data = get_service_data(Some(claims), true)?;
    let session_id = data.get_session_id();
    let mut ul = undo.lock().map_err(|e| BpError::from(e.to_string()))?;
    let us = (*ul)
        .map
        .entry(session_id.clone())
        .or_insert(service::UndoRedoStack::new(session_id));
    let undolist0 = us.get_last_undo();
    data.ul.add_list(&undolist0);
    let (ulist, ul2) = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut con = pool.get()?;
        service::client::undo(&mut con, &mut data)?; //, undolist)?;
        let ulist = service::client::get_user_list(&mut con, &mut data, true, false)?;
        Ok((ulist, data.ul)) as Result<(Vec<Benutzer>, UndoList), BpError>
    })
    .await??;
    us.remove_undo(&ul2);
    Ok(HttpResponse::Ok().json(ulist))
}

#[get("/r")]
pub async fn listr(
    claims: Claims,
    pool: web::Data<DbPool>,
    undo: web::Data<Mutex<UndoPool>>,
) -> Result<impl Responder, BpError> {
    let mut data = get_service_data(Some(claims), true)?;
    let session_id = data.get_session_id();
    let mut ul = undo.lock().map_err(|e| BpError::from(e.to_string()))?;
    let us = (*ul)
        .map
        .entry(session_id.clone())
        .or_insert(service::UndoRedoStack::new(session_id));
    let undolist0 = us.get_last_redo();
    data.ul.add_list(&undolist0);
    let (ulist, ul2) = web::block(move || {
        let mut con = pool.get()?;
        service::client::redo(&mut con, &mut data)?;
        let ulist = service::client::get_user_list(&mut con, &mut data, true, false)?;
        Ok((ulist, data.ul)) as Result<(Vec<Benutzer>, UndoList), BpError>
    })
    .await??;
    us.remove_redo(&ul2);
    Ok(HttpResponse::Ok().json(ulist))
}
