use crate::{
    api::{get_service_data, session_undo},
    base::{BpError, DbPool, UndoPool},
    //extractors::Claims,
};
use actix_web::{get, web, HttpResponse, Responder, Result};
use basis::functions;
use rep::models::TbEintrag;
use service::UndoList;
use std::sync::Mutex;

#[get("/last")]
pub async fn last(
    // TODO claims: Claims,
    pool: web::Data<DbPool>,
    undo: web::Data<Mutex<UndoPool>>,
) -> Result<impl Responder, BpError> {
    functions::mach_nichts();
    // let mut data = get_service_data(Some(claims), true)?;
    let mut data = get_service_data(None, true)?;
    let session_id = data.get_session_id();
    let f = move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut con = pool.get()?;
        let ulist = service::diary::get_last_entries(&mut con, &mut data)?;
        Ok((ulist, data.ul)) as Result<(Vec<TbEintrag>, UndoList), BpError>
    };
    let ulist = session_undo(session_id, undo, f).await?;
    Ok(HttpResponse::Ok().json(ulist))
}

#[get("/{date}")] // yyyy-mm-dd
pub async fn list(
    // TODO claims: Claims,
    path: web::Path<String>,
    pool: web::Data<DbPool>,
    undo: web::Data<Mutex<UndoPool>>,
) -> Result<impl Responder, BpError> {
    // let mut data = get_service_data(Some(claims), true)?;
    let date = path.into_inner();
    let mut data = get_service_data(None, true)?;
    let session_id = data.get_session_id();
    let date = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap_or(data.get_today());
    let f = move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut con = pool.get()?;
        let ulist = service::diary::get_entries(&mut con, &mut data, &date, 3)?;
        Ok((ulist, data.ul)) as Result<(Vec<Option<TbEintrag>>, UndoList), BpError>
    };
    let ulist = session_undo(session_id, undo, f).await?;
    Ok(HttpResponse::Ok().json(ulist))
}
