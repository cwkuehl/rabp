use crate::{
    base::{
        errors::Result,
        reps::{mach_angelegt, mach_geaendert},
        undo::UndoEntry,
    },
    ServiceData, ServiceError,
};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{prelude::*, SqliteConnection};
use rep::{models::TbEintrag, schema::TB_EINTRAG};

/// Undoes dataset.
pub fn undo(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    or: &String,
    ac: &String,
) -> Result<()> {
    let oo = UndoEntry::from_str::<TbEintrag>(or)?;
    let oa = UndoEntry::from_str::<TbEintrag>(ac)?;
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
    let oo = UndoEntry::from_str::<TbEintrag>(or)?;
    let oa = UndoEntry::from_str::<TbEintrag>(ac)?;
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
    datum_: &NaiveDate,
    eintrag_: &String,
    angelegt_von_: &Option<String>,
    angelegt_am_: &Option<NaiveDateTime>,
    geaendert_von_: &Option<String>,
    geaendert_am_: &Option<NaiveDateTime>,
    replikation_uid_: &Option<String>,
) -> Result<TbEintrag> {
    let op = TB_EINTRAG::table
        .filter(
            TB_EINTRAG::mandant_nr
                .eq(mandant_nr_)
                .and(TB_EINTRAG::datum.eq(datum_.clone())),
        )
        .first::<TbEintrag>(con)
        .optional()?;
    let mut p = TbEintrag {
        mandant_nr: *mandant_nr_,
        datum: datum_.clone(),
        eintrag: eintrag_.clone(),
        angelegt_von: None,
        angelegt_am: None,
        geaendert_von: None,
        geaendert_am: None,
        replikation_uid: replikation_uid_.clone(),
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
    datum_: &NaiveDate,
    eintrag_: &String,
) -> Result<TbEintrag> {
    save0(
        con,
        data,
        mandant_nr_,
        datum_,
        eintrag_,
        &None,
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
    datum_: &NaiveDate,
) -> Result<Option<TbEintrag>> {
    let p = TB_EINTRAG::table
        .filter(
            TB_EINTRAG::mandant_nr
                .eq(mandant_nr_)
                .and(TB_EINTRAG::datum.eq(datum_.clone())),
        )
        .first::<TbEintrag>(con)
        .optional()?;
    Ok(p)
}

/// Gets dataset by primary key.
pub fn get2(con: &mut SqliteConnection, b: &TbEintrag) -> Result<Option<TbEintrag>> {
    let p = TB_EINTRAG::table
        .filter(
            TB_EINTRAG::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_EINTRAG::datum.eq(b.datum.clone())),
        )
        .first::<TbEintrag>(con)
        .optional()?;
    Ok(p)
}

/// Gets list.
#[allow(dead_code)]
pub fn get_list(con: &mut SqliteConnection, mandant_nr_: i32) -> Result<Vec<TbEintrag>> {
    let list = TB_EINTRAG::table
        .filter(TB_EINTRAG::mandant_nr.eq(mandant_nr_))
        .load::<TbEintrag>(con)?;
    Ok(list)
}

/// Inserts dataset.
pub fn insert<'a>(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    b: &'a TbEintrag,
) -> Result<&'a TbEintrag> {
    let rows = diesel::insert_into(TB_EINTRAG::table)
        .values(b)
        .execute(con)?;
    if rows <= 0 {
        return Err(ServiceError::NotFound);
    }
    data.ul.add(&UndoEntry::tb_eintrag(None, Some(b)));
    Ok(b)
}

/// Updates dataset.
pub fn update<'a>(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    b: &'a TbEintrag,
) -> Result<&'a TbEintrag> {
    let oo = get2(con, b)?;
    let rows = diesel::update(
        TB_EINTRAG::table.filter(
            TB_EINTRAG::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_EINTRAG::datum.eq(b.datum.clone())),
        ),
    )
    .set((
        TB_EINTRAG::eintrag.eq(b.eintrag.as_str()),
        TB_EINTRAG::angelegt_von.eq(b.angelegt_von.as_ref()),
        TB_EINTRAG::angelegt_am.eq(b.angelegt_am),
        TB_EINTRAG::geaendert_von.eq(b.geaendert_von.as_ref()),
        TB_EINTRAG::geaendert_am.eq(b.geaendert_am),
        TB_EINTRAG::replikation_uid.eq(b.replikation_uid.as_ref()),
    ))
    .execute(con)?;
    if rows <= 0 || oo.is_none() {
        return Err(ServiceError::NotFound);
    }
    if let Some(o) = oo {
        data.ul.add(&UndoEntry::tb_eintrag(Some(&o), Some(b)));
    }
    Ok(b)
}

/// Deletes dataset.
pub fn delete(con: &mut SqliteConnection, data: &mut ServiceData, b: &TbEintrag) -> Result<()> {
    let oo = get2(con, b)?;
    let rows = diesel::delete(
        TB_EINTRAG::table.filter(
            TB_EINTRAG::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_EINTRAG::datum.eq(b.datum.clone())),
        ),
    )
    .execute(con)?;
    if rows <= 0 || oo.is_none() {
        return Err(ServiceError::NotFound);
    }
    if let Some(o) = oo {
        data.ul.add(&UndoEntry::tb_eintrag(Some(&o), None));
    }
    Ok(())
}

/// Gets list with optional date and limit.
/// * `con` - Affected database connection.
/// * `mandant_nr_` - Affected client number.
/// * `date` - Date is filtered by lower equal or None.
/// * `limit` - Limit of data rows.
#[allow(dead_code)]
pub fn get_list2(
    con: &mut SqliteConnection,
    mandant_nr_: i32,
    date: Option<&NaiveDate>,
    limit: i64,
) -> Result<Vec<TbEintrag>> {
    let mut q = TB_EINTRAG::table
        .into_boxed()
        .filter(TB_EINTRAG::mandant_nr.eq(mandant_nr_));
    if let Some(d) = date {
        q = q.filter(TB_EINTRAG::datum.le(d));
    }
    let list = q
        .order(TB_EINTRAG::datum.desc())
        .limit(limit)
        .load::<TbEintrag>(con)?;
    Ok(list)
}
