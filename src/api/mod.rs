mod diary;
mod messages;
mod routes;
mod users;

pub use self::routes::routes;

use crate::{
    base::{BpError, UndoPool},
    extractors::Claims,
};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};
use service::{ServiceError, UndoList};
use std::{collections::HashSet, sync::Mutex};

#[cfg(not(rust_analyzer))]
const BUILD_TIME: &str = include!(concat!(env!("OUT_DIR"), "/timestamp.txt"));

#[cfg(rust_analyzer)]
const BUILD_TIME: &str = "BUILD_TIME_UNAVAILABLE";

#[get("/version")]
pub async fn version(_req: HttpRequest) -> Result<impl Responder, BpError> {
    // Ok(HttpResponse::Ok().json("24.08.2025"))
    Ok(HttpResponse::Ok().json(BUILD_TIME)) // returns '"2025-08-24 17:18:59.144916399 +02:00"'
}

#[get("/health")]
pub async fn health() -> Result<impl Responder, BpError> {
    Ok(HttpResponse::Ok())
}

pub fn get_service_data(
    claims: Option<Claims>,
    admin: bool,
) -> Result<service::ServiceData, BpError> {
    let mut clientid: i32 = 1;
    let mut userid: String = "unknown".to_string();
    if let Some(claims) = claims {
        clientid = claims.get_client_id();
        userid = claims.get_user_id();
        if (clientid < 0) || (userid == "") {
            return Err(BpError::PermissionError);
        }
        if admin {
            let perm = "perm:admin1";
            if !claims.validate_permissions(&HashSet::from([perm.to_string()])) {
                return Err(BpError::PermissionError);
            }
        }
    }
    let data = service::ServiceData::new(clientid, &userid);
    Ok(data)
}

/// Surrounding function for session handling and undo pool logic.
pub async fn session_undo<F, R>(
    session_id: String,
    undo: web::Data<Mutex<UndoPool>>,
    f: F,
) -> Result<R, BpError>
where
    F: FnOnce() -> Result<(R, UndoList), BpError> + Send + 'static,
    R: Send + 'static,
{
    // First: Session id is used to identify the undo stack.
    // let session_id = get_session_id(&session)?;
    let sid = session_id.clone();
    // Second: Doing the actual work.
    let (r, ul) = f()?;
    // Third: Optimistic locking the undo pool to get the undo stack.
    // let uplock = undo.lock().map_err(|e| BpError::from(e.to_string()))?;
    match undo.lock() {
        Err(e) => {
            // TODO Undo if mutex has error.
            // let (ulist, ul2) = web::block(move || {
            //     let mut con = pool.get()?;
            //     service::client::undo(&mut con, &mut data)?; //, undolist)?;
            //     Ok((ulist, data.ul)) as Result<(Vec<Benutzer>, UndoList), BpError>
            // })
            // .await??;
            return Err(BpError::from(e.to_string()));
        }
        Ok(mut uplock) => {
            let urstack = (*uplock)
                .map
                .entry(session_id)
                .or_insert(service::UndoRedoStack::new(sid));
            urstack.add_undo(&ul);
        }
    }
    Ok(r)
}

/// Checks if the request comes from a local address.
pub fn is_local(req: &HttpRequest) -> Result<(), ServiceError> {
    if let Some(val) = req.peer_addr() {
        let adr = val.ip().to_string();
        println!("Request from address {:?}", adr);
        if !adr.starts_with("127.0.0.1")
            && !adr.starts_with("192.168.8.128")
            && !adr.starts_with("::1")
        {
            return Err(ServiceError::error_string(
                format!("Forbidden: {}", adr).as_str(),
            ));
        }
    };
    Ok(())
}
