use crate::{
    base::{
        errors::Result,
        reps::{mach_angelegt, mach_geaendert},
        undo::UndoEntry,
    },
    ServiceData, ServiceError,
};
use basis::functions;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{prelude::*, SqliteConnection};
use rep::{
    models::{TbEintragOrt, TbOrt},
    models_ext::TbEintragOrtExt,
    schema::{TB_EINTRAG_ORT, TB_ORT},
};

/// Undoes dataset.
pub fn undo(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    or: &String,
    ac: &String,
) -> Result<()> {
    let oo = UndoEntry::from_str::<TbEintragOrt>(or)?;
    let oa = UndoEntry::from_str::<TbEintragOrt>(ac)?;
    if let (Some(o), Some(_a)) = (&oo, &oa) {
        // Update
        update(con, data, o)?;
    } else if let Some(a) = &oa {
        // Insert
        delete(con, data, a)?;
    } else if let Some(o) = &oo {
        // Delete
        insert(con, data, o)?;
    }
    Ok(())
}

/// Redoes dataset.
pub fn redo(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    or: &String,
    ac: &String,
) -> Result<()> {
    let oo = UndoEntry::from_str::<TbEintragOrt>(or)?;
    let oa = UndoEntry::from_str::<TbEintragOrt>(ac)?;
    if let (Some(_o), Some(a)) = (&oo, &oa) {
        // Update
        update(con, data, a)?;
    } else if let Some(a) = &oa {
        // Insert
        insert(con, data, a)?;
    } else if let Some(o) = &oo {
        // Delete
        delete(con, data, o)?;
    }
    Ok(())
}

/// Saves dataset with all values.
#[allow(dead_code)]
pub fn save0(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    mandant_nr_: &i32,
    ort_uid_: &String,
    datum_von_: &NaiveDate,
    datum_bis_: &NaiveDate,
    angelegt_von_: &Option<String>,
    angelegt_am_: &Option<NaiveDateTime>,
    geaendert_von_: &Option<String>,
    geaendert_am_: &Option<NaiveDateTime>,
) -> Result<TbEintragOrt> {
    let op = TB_EINTRAG_ORT::table
        .filter(
            TB_EINTRAG_ORT::mandant_nr
                .eq(mandant_nr_)
                .and(TB_EINTRAG_ORT::ort_uid.eq(ort_uid_.clone()))
                .and(TB_EINTRAG_ORT::datum_von.eq(datum_von_.clone()))
                .and(TB_EINTRAG_ORT::datum_bis.eq(datum_bis_.clone())),
        )
        .first::<TbEintragOrt>(con)
        .optional()?;
    let mut p = TbEintragOrt {
        mandant_nr: *mandant_nr_,
        ort_uid: ort_uid_.clone(),
        datum_von: datum_von_.clone(),
        datum_bis: datum_bis_.clone(),
        angelegt_von: None,
        angelegt_am: None,
        geaendert_von: None,
        geaendert_am: None,
    };
    if let Some(pu) = op {
        if p != pu {
            p.angelegt_von = pu.angelegt_von;
            p.angelegt_am = pu.angelegt_am;
            p.geaendert_von = pu.geaendert_von;
            p.geaendert_am = pu.geaendert_am;
            if p.angelegt_von.is_none() || !angelegt_von_.is_none() {
                mach_angelegt(&mut p, data, angelegt_von_, angelegt_am_);
            }
            mach_geaendert(&mut p, data, geaendert_von_, geaendert_am_);
            update(con, data, &p)?;
        }
    } else {
        mach_angelegt(&mut p, data, angelegt_von_, angelegt_am_);
        if !geaendert_von_.is_none() {
            mach_geaendert(&mut p, data, geaendert_von_, geaendert_am_);
        }
        insert(con, data, &p)?;
    }
    return Ok(p);
}

/// Saves dataset without revision columns.
#[allow(dead_code)]
pub fn save(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    mandant_nr_: &i32,
    ort_uid_: &String,
    datum_von_: &NaiveDate,
    datum_bis_: &NaiveDate,
) -> Result<TbEintragOrt> {
    save0(
        con,
        data,
        mandant_nr_,
        ort_uid_,
        datum_von_,
        datum_bis_,
        &None,
        &None,
        &None,
        &None,
    )
}

/// Gets dataset by primary key.
#[allow(dead_code)]
pub fn get(
    con: &mut SqliteConnection,
    mandant_nr_: &i32,
    ort_uid_: &String,
    datum_von_: &NaiveDate,
    datum_bis_: &NaiveDate,
) -> Result<Option<TbEintragOrt>> {
    let p = TB_EINTRAG_ORT::table
        .filter(
            TB_EINTRAG_ORT::mandant_nr
                .eq(mandant_nr_)
                .and(TB_EINTRAG_ORT::ort_uid.eq(ort_uid_.clone()))
                .and(TB_EINTRAG_ORT::datum_von.eq(datum_von_.clone()))
                .and(TB_EINTRAG_ORT::datum_bis.eq(datum_bis_.clone())),
        )
        .first::<TbEintragOrt>(con)
        .optional()?;
    Ok(p)
}

/// Gets dataset by primary key.
pub fn get2(con: &mut SqliteConnection, b: &TbEintragOrt) -> Result<Option<TbEintragOrt>> {
    let p = TB_EINTRAG_ORT::table
        .filter(
            TB_EINTRAG_ORT::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_EINTRAG_ORT::ort_uid.eq(b.ort_uid.clone()))
                .and(TB_EINTRAG_ORT::datum_von.eq(b.datum_von.clone()))
                .and(TB_EINTRAG_ORT::datum_bis.eq(b.datum_bis.clone())),
        )
        .first::<TbEintragOrt>(con)
        .optional()?;
    Ok(p)
}

/// Gets list.
#[allow(dead_code)]
pub fn get_list(con: &mut SqliteConnection, mandant_nr_: i32) -> Result<Vec<TbEintragOrt>> {
    let list = TB_EINTRAG_ORT::table
        .filter(TB_EINTRAG_ORT::mandant_nr.eq(mandant_nr_))
        .load::<TbEintragOrt>(con)?;
    Ok(list)
}

/// Inserts dataset.
pub fn insert<'a>(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    b: &'a TbEintragOrt,
) -> Result<&'a TbEintragOrt> {
    let rows = diesel::insert_into(TB_EINTRAG_ORT::table)
        .values(b)
        .execute(con)?;
    if rows <= 0 {
        return Err(ServiceError::NotFound);
    }
    data.ul.add(&UndoEntry::tb_eintrag_ort(None, Some(b)));
    Ok(b)
}

/// Updates dataset.
pub fn update<'a>(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    b: &'a TbEintragOrt,
) -> Result<&'a TbEintragOrt> {
    let oo = get2(con, b)?;
    let rows = diesel::update(
        TB_EINTRAG_ORT::table.filter(
            TB_EINTRAG_ORT::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_EINTRAG_ORT::ort_uid.eq(b.ort_uid.clone()))
                .and(TB_EINTRAG_ORT::datum_von.eq(b.datum_von.clone()))
                .and(TB_EINTRAG_ORT::datum_bis.eq(b.datum_bis.clone())),
        ),
    )
    .set((
        TB_EINTRAG_ORT::angelegt_von.eq(b.angelegt_von.as_ref()),
        TB_EINTRAG_ORT::angelegt_am.eq(b.angelegt_am),
        TB_EINTRAG_ORT::geaendert_von.eq(b.geaendert_von.as_ref()),
        TB_EINTRAG_ORT::geaendert_am.eq(b.geaendert_am),
    ))
    .execute(con)?;
    if rows <= 0 || oo.is_none() {
        return Err(ServiceError::NotFound);
    }
    if let Some(o) = oo {
        data.ul.add(&UndoEntry::tb_eintrag_ort(Some(&o), Some(b)));
    }
    Ok(b)
}

/// Deletes dataset.
pub fn delete(con: &mut SqliteConnection, data: &mut ServiceData, b: &TbEintragOrt) -> Result<()> {
    let oo = get2(con, b)?;
    let rows = diesel::delete(
        TB_EINTRAG_ORT::table.filter(
            TB_EINTRAG_ORT::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_EINTRAG_ORT::ort_uid.eq(b.ort_uid.clone()))
                .and(TB_EINTRAG_ORT::datum_von.eq(b.datum_von.clone()))
                .and(TB_EINTRAG_ORT::datum_bis.eq(b.datum_bis.clone())),
        ),
    )
    .execute(con)?;
    if rows <= 0 || oo.is_none() {
        return Err(ServiceError::NotFound);
    }
    if let Some(o) = oo {
        data.ul.add(&UndoEntry::tb_eintrag_ort(Some(&o), None));
    }
    Ok(())
}

/// Gets list.
pub fn get_list_ext2(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    date: &NaiveDate,
) -> Result<Vec<TbEintragOrtExt>> {
    let join = TB_EINTRAG_ORT::table
        .filter(
            TB_EINTRAG_ORT::mandant_nr.eq(data.mandant_nr).and(
                TB_EINTRAG_ORT::datum_von
                    .le(date)
                    .and(TB_EINTRAG_ORT::datum_bis.ge(date)),
            ),
        )
        .inner_join(
            TB_ORT::table.on(TB_ORT::mandant_nr
                .eq(TB_EINTRAG_ORT::mandant_nr)
                .and(TB_ORT::uid.eq(TB_EINTRAG_ORT::ort_uid))),
        )
        .order_by((
            TB_EINTRAG_ORT::mandant_nr,
            TB_EINTRAG_ORT::ort_uid,
            TB_EINTRAG_ORT::datum_von,
        ))
        .load::<(TbEintragOrt, TbOrt)>(con)?;
    let mut l: Vec<TbEintragOrtExt> = Vec::new();
    for j in join {
        l.push(TbEintragOrtExt {
            mandant_nr: j.0.mandant_nr,
            ort_uid: j.0.ort_uid,
            datum_von: j.0.datum_von,
            datum_bis: j.0.datum_bis,
            angelegt_von: j.0.angelegt_von,
            angelegt_am: j.0.angelegt_am,
            geaendert_von: j.0.geaendert_von,
            geaendert_am: j.0.geaendert_am,
            bezeichnung: j.1.bezeichnung,
            breite: j.1.breite,
            laenge: j.1.laenge,
            hoehe: j.1.hoehe,
            notiz: j.1.notiz,
        });
    }
    Ok(l)
}

/// Gets list extension.
pub fn get_list_ext(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    from: Option<&NaiveDate>,
    add_days: &i32,
    to: Option<&NaiveDate>,
    puid: Option<&String>,
) -> Result<Vec<TbEintragOrt>> {
    let mut q = TB_EINTRAG_ORT::table
        .into_boxed()
        .filter(TB_EINTRAG_ORT::mandant_nr.eq(data.mandant_nr));
    if let Some(date) = from {
        let f = functions::nd_add_dmy(date, *add_days, 0, 0).unwrap_or(*date);
        if let Some(t) = to {
            q = q.filter(
                TB_EINTRAG_ORT::datum_von
                    .le(t)
                    .and(TB_EINTRAG_ORT::datum_bis.ge(f)),
            );
        } else {
            q = q.filter(
                TB_EINTRAG_ORT::datum_von
                    .le(f)
                    .and(TB_EINTRAG_ORT::datum_bis.ge(f)),
            );
        }
    }
    if let Some(id) = puid {
        q = q.filter(TB_EINTRAG_ORT::ort_uid.eq(id));
    }
    let list = q
        .order_by((
            TB_EINTRAG_ORT::mandant_nr,
            TB_EINTRAG_ORT::ort_uid,
            TB_EINTRAG_ORT::datum_von,
        ))
        .load::<TbEintragOrt>(con)?;
    Ok(list)
}
