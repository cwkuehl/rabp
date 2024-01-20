use crate::{
    api::{get_service_data, is_local, session_undo},
    base::{BpError, DbPool, UndoPool},
    extractors::Claims,
};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder, Result};
use basis::functions;
use rep::models::TbEintrag;
use serde::Deserialize;
use service::UndoList;
use std::sync::Mutex;

#[get("/last")]
pub async fn last(
    claims: Claims,
    pool: web::Data<DbPool>,
    undo: web::Data<Mutex<UndoPool>>,
) -> Result<impl Responder, BpError> {
    functions::mach_nichts();
    let mut data = get_service_data(Some(claims), true)?;
    // let mut data = get_service_data(None, true)?;
    let session_id = data.get_session_id();
    let f = move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut con = pool.get()?;
        let r = service::diary::get_last_entries(&mut con, &mut data)?;
        Ok((r, data.ul)) as Result<(Vec<TbEintrag>, UndoList), BpError>
    };
    let r = session_undo(session_id, undo, f).await?;
    Ok(HttpResponse::Ok().json(r))
}

#[get("/listlocal/{date}")] // date: yyyy-mm-dd;
pub async fn list_local(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
    undo: web::Data<Mutex<UndoPool>>,
    req: HttpRequest,
) -> Result<impl Responder, BpError> {
    is_local(&req)?;
    let date = path.into_inner();
    let count = -1;
    // let mut data = get_service_data(Some(claims), true)?;
    let mut data = get_service_data(None, true)?;
    let session_id = data.get_session_id();
    let date = functions::to_date(&date, &data.get_today());
    let f = move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut con = pool.get()?;
        let r = service::diary::get_entries(&mut con, &mut data, &date, count)?;
        Ok((r, data.ul)) as Result<(Vec<Option<TbEintrag>>, UndoList), BpError>
    };
    let r = session_undo(session_id, undo, f).await?;
    Ok(HttpResponse::Ok().json(r))
}

/// Deletes all entries before the given date.
#[delete("/{date}")] // date: yyyy-mm-dd;
pub async fn delete_local(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
    undo: web::Data<Mutex<UndoPool>>,
    req: HttpRequest,
) -> Result<impl Responder, BpError> {
    is_local(&req)?;
    let date = path.into_inner();
    // let mut data = get_service_data(Some(claims), true)?;
    let mut data = get_service_data(None, true)?;
    let session_id = data.get_session_id();
    let date = functions::to_date(&date, &data.get_today());
    let f = move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut con = pool.get()?;
        let r = service::diary::delete_entries(&mut con, &mut data, &date)?;
        Ok((r, data.ul)) as Result<((), UndoList), BpError>
    };
    let r = session_undo(session_id, undo, f).await?;
    Ok(HttpResponse::Ok().json(r))
}

#[get("/list/{date}/{count}")] // date: yyyy-mm-dd; count: 1, 3, 5 or 7
pub async fn list(
    claims: Claims,
    path: web::Path<(String, u32)>,
    pool: web::Data<DbPool>,
    undo: web::Data<Mutex<UndoPool>>,
) -> Result<impl Responder, BpError> {
    let (date, count) = path.into_inner();
    let mut data = get_service_data(Some(claims), true)?;
    // let mut data = get_service_data(None, true)?;
    let session_id = data.get_session_id();
    let date = functions::to_date(&date, &data.get_today());
    let f = move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut con = pool.get()?;
        let r = service::diary::get_entries(&mut con, &mut data, &date, count as i32)?;
        Ok((r, data.ul)) as Result<(Vec<Option<TbEintrag>>, UndoList), BpError>
    };
    let r = session_undo(session_id, undo, f).await?;
    Ok(HttpResponse::Ok().json(r))
}

#[derive(Deserialize)]
pub struct DiaryEntry {
    date: String, // Format yyyy-mm-dd
    entry: String,
}

#[post("/")]
pub async fn save(
    claims: Claims,
    json: web::Json<DiaryEntry>,
    pool: web::Data<DbPool>,
    undo: web::Data<Mutex<UndoPool>>,
) -> Result<impl Responder, BpError> {
    let mut data = get_service_data(Some(claims), true)?;
    // let mut data = get_service_data(None, true)?;
    let session_id = data.get_session_id();
    let date = functions::to_date(&json.date, &data.get_today());
    let plist = vec![]; // TODO TbEintragOrtExt
    let f = move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut con = pool.get()?;
        let r = service::diary::save_entry(&mut con, &mut data, &date, &json.entry, &plist, true)?;
        Ok((r, data.ul)) as Result<((), UndoList), BpError>
    };
    let r = session_undo(session_id, undo, f).await?;
    Ok(HttpResponse::Ok().json(r))
}

#[get("/undo")]
pub async fn do_undo(
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
    let ul2 = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut con = pool.get()?;
        service::client::undo(&mut con, &mut data)?;
        Ok(data.ul) as Result<UndoList, BpError>
    })
    .await??;
    us.remove_undo(&ul2);
    Ok(HttpResponse::Ok())
}

#[get("/redo")]
pub async fn do_redo(
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
    let ul2 = web::block(move || {
        let mut con = pool.get()?;
        service::client::redo(&mut con, &mut data)?;
        Ok(data.ul) as Result<UndoList, BpError>
    })
    .await??;
    us.remove_redo(&ul2);
    Ok(HttpResponse::Ok())
}
