use crate::{
    api::{get_service_data, session_undo},
    base::{BpError, DbPool, UndoPool},
    extractors::Claims,
};
use actix_web::{get, web, HttpResponse, Responder, Result};
use basis::functions;
use rep::models::Benutzer;
use service::UndoList;
use std::sync::Mutex;

#[get("")]
pub async fn list(
    claims: Claims,
    pool: web::Data<DbPool>,
    undo: web::Data<Mutex<UndoPool>>,
) -> Result<impl Responder, BpError> {
    functions::mach_nichts();
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