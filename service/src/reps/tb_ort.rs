use crate::{
    base::{
        errors::Result,
        reps::{mach_angelegt, mach_geaendert},
        undo::UndoEntry,
    },
    ServiceData, ServiceError,
};
use chrono::NaiveDateTime;
use diesel::{prelude::*, SqliteConnection};
use rep::{models::TbOrt, schema::TB_ORT};

/// Undo a dataset.
pub fn undo(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    or: &String,
    ac: &String,
) -> Result<()> {
    let oo = UndoEntry::from_str::<TbOrt>(or)?;
    let oa = UndoEntry::from_str::<TbOrt>(ac)?;
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

/// Redo a dataset.
pub fn redo(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    or: &String,
    ac: &String,
) -> Result<()> {
    let oo = UndoEntry::from_str::<TbOrt>(or)?;
    let oa = UndoEntry::from_str::<TbOrt>(ac)?;
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

/// Save dataset with all values.
#[allow(dead_code)]
pub fn save0(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    mandant_nr_: &i32,
    uid_: &String,
    bezeichnung_: &String,
    breite_: &f64,
    laenge_: &f64,
    hoehe_: &f64,
    zeitzone_: &Option<String>,
    notiz_: &Option<String>,
    angelegt_von_: &Option<String>,
    angelegt_am_: &Option<NaiveDateTime>,
    geaendert_von_: &Option<String>,
    geaendert_am_: &Option<NaiveDateTime>,
) -> Result<TbOrt> {
    let op = TB_ORT::table
        .filter(
            TB_ORT::mandant_nr
                .eq(mandant_nr_)
                .and(TB_ORT::uid.eq(uid_.clone())),
        )
        .first::<TbOrt>(con)
        .optional()?;
    let mut p = TbOrt {
        mandant_nr: *mandant_nr_,
        uid: uid_.clone(),
        bezeichnung: bezeichnung_.clone(),
        breite: *breite_,
        laenge: *laenge_,
        hoehe: *hoehe_,
        zeitzone: zeitzone_.clone(),
        notiz: notiz_.clone(),
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

/// Save dataset without revision columns.
#[allow(dead_code)]
pub fn save(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    mandant_nr_: &i32,
    uid_: &String,
    bezeichnung_: &String,
    breite_: &f64,
    laenge_: &f64,
    hoehe_: &f64,
    zeitzone_: &Option<String>,
    notiz_: &Option<String>,
) -> Result<TbOrt> {
    save0(
        con,
        data,
        mandant_nr_,
        uid_,
        bezeichnung_,
        breite_,
        laenge_,
        hoehe_,
        zeitzone_,
        notiz_,
        &None,
        &None,
        &None,
        &None,
    )
}

/// Get dataset by primary key.
#[allow(dead_code)]
pub fn get(con: &mut SqliteConnection, mandant_nr_: &i32, uid_: &String) -> Result<Option<TbOrt>> {
    let p = TB_ORT::table
        .filter(
            TB_ORT::mandant_nr
                .eq(mandant_nr_)
                .and(TB_ORT::uid.eq(uid_.clone())),
        )
        .first::<TbOrt>(con)
        .optional()?;
    Ok(p)
}

/// Get dataset by primary key.
pub fn get2(con: &mut SqliteConnection, b: &TbOrt) -> Result<Option<TbOrt>> {
    let p = TB_ORT::table
        .filter(
            TB_ORT::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_ORT::uid.eq(b.uid.clone())),
        )
        .first::<TbOrt>(con)
        .optional()?;
    Ok(p)
}

/// Get list.
#[allow(dead_code)]
pub fn get_list(con: &mut SqliteConnection, mandant_nr_: i32) -> Result<Vec<TbOrt>> {
    let list = TB_ORT::table
        .filter(TB_ORT::mandant_nr.eq(mandant_nr_))
        .load::<TbOrt>(con)?;
    Ok(list)
}

/// Insert a dataset.
pub fn insert<'a>(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    b: &'a TbOrt,
) -> Result<&'a TbOrt> {
    let rows = diesel::insert_into(TB_ORT::table).values(b).execute(con)?;
    if rows <= 0 {
        return Err(ServiceError::NotFound);
    }
    data.ul.add(&UndoEntry::tb_ort(None, Some(b)));
    Ok(b)
}

/// Update a dataset.
pub fn update<'a>(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    b: &'a TbOrt,
) -> Result<&'a TbOrt> {
    let oo = get2(con, b)?;
    let rows = diesel::update(
        TB_ORT::table.filter(
            TB_ORT::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_ORT::uid.eq(b.uid.clone())),
        ),
    )
    .set((
        TB_ORT::bezeichnung.eq(b.bezeichnung.as_str()),
        TB_ORT::breite.eq(b.breite),
        TB_ORT::laenge.eq(b.laenge),
        TB_ORT::hoehe.eq(b.hoehe),
        TB_ORT::zeitzone.eq(b.zeitzone.as_ref()),
        TB_ORT::notiz.eq(b.notiz.as_ref()),
        TB_ORT::angelegt_von.eq(b.angelegt_von.as_ref()),
        TB_ORT::angelegt_am.eq(b.angelegt_am),
        TB_ORT::geaendert_von.eq(b.geaendert_von.as_ref()),
        TB_ORT::geaendert_am.eq(b.geaendert_am),
    ))
    .execute(con)?;
    if rows <= 0 || oo.is_none() {
        return Err(ServiceError::NotFound);
    }
    if let Some(o) = oo {
        data.ul.add(&UndoEntry::tb_ort(Some(&o), Some(b)));
    }
    Ok(b)
}

/// Delete a dataset.
pub fn delete(con: &mut SqliteConnection, data: &mut ServiceData, b: &TbOrt) -> Result<()> {
    let oo = get2(con, b)?;
    let rows = diesel::delete(
        TB_ORT::table.filter(
            TB_ORT::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_ORT::uid.eq(b.uid.clone())),
        ),
    )
    .execute(con)?;
    if rows <= 0 || oo.is_none() {
        return Err(ServiceError::NotFound);
    }
    if let Some(o) = oo {
        data.ul.add(&UndoEntry::tb_ort(Some(&o), None));
    }
    Ok(())
}
