use crate::base::{errors::Result, undo::UndoEntry};
use crate::{ServiceData, ServiceError};
use rep::models::Benutzer;
use rep::schema::BENUTZER;

// use super::DbContext;
// use crate::{config::RsbpError, services::undo::UndoEntry, Result};
// use chrono::{NaiveDate, NaiveDateTime};
use diesel::{prelude::*, SqliteConnection};
// use rep::{models::Benutzer, schema::*};

// /// Undo a dataset.
// pub fn undo(db: &mut DbContext, or: &String, ac: &String) -> Result<()> {
//     let oo = UndoEntry::from_str::<Benutzer>(or)?;
//     let oa = UndoEntry::from_str::<Benutzer>(ac)?;
//     if let (Some(o), Some(_a)) = (&oo, &oa) {
//         // Update
//         update(db, o)?;
//     } else if let Some(a) = &oa {
//         // Insert
//         delete(db, a)?;
//     } else if let Some(o) = &oo {
//         // Delete
//         insert(db, o)?;
//     }
//     Ok(())
// }

// /// Redo a dataset.
// pub fn redo(db: &mut DbContext, or: &String, ac: &String) -> Result<()> {
//     let oo = UndoEntry::from_str::<Benutzer>(or)?;
//     let oa = UndoEntry::from_str::<Benutzer>(ac)?;
//     if let (Some(_o), Some(a)) = (&oo, &oa) {
//         // Update
//         update(db, a)?;
//     } else if let Some(a) = &oa {
//         // Insert
//         insert(db, a)?;
//     } else if let Some(o) = &oo {
//         // Delete
//         delete(db, o)?;
//     }
//     Ok(())
// }

// /// Save dataset with all values.
// #[allow(dead_code)]
// pub fn save0(
//     db: &mut DbContext,
//     mandant_nr_: &i32,
//     benutzer_id_: &String,
//     passwort_: &Option<String>,
//     berechtigung_: &i32,
//     akt_periode_: &i32,
//     person_nr_: &i32,
//     geburt_: &Option<NaiveDate>,
//     angelegt_von_: &Option<String>,
//     angelegt_am_: &Option<NaiveDateTime>,
//     geaendert_von_: &Option<String>,
//     geaendert_am_: &Option<NaiveDateTime>
// ) -> Result<Benutzer> {
//     let op = BENUTZER::table
//         .filter(
//             BENUTZER::mandant_nr.eq(mandant_nr_)
//             .and(BENUTZER::benutzer_id.eq(benutzer_id_.clone())),
//         )
//         .first::<Benutzer>(db.c)
//         .optional()
//         .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
//     let mut p = Benutzer {
//         mandant_nr: *mandant_nr_,
//         benutzer_id: benutzer_id_.clone(),
//         passwort: passwort_.clone(),
//         berechtigung: *berechtigung_,
//         akt_periode: *akt_periode_,
//         person_nr: *person_nr_,
//         geburt: geburt_.clone(),
//         angelegt_von: None,
//         angelegt_am: None,
//         geaendert_von: None,
//         geaendert_am: None,
//     };
//     if let Some(pu) = op {
//         if p != pu {
//         p.angelegt_von = pu.angelegt_von;
//         p.angelegt_am = pu.angelegt_am;
//         p.geaendert_von = pu.geaendert_von;
//         p.geaendert_am = pu.geaendert_am;
//         if p.angelegt_von.is_none() || !angelegt_von_.is_none() {
//                 super::mach_angelegt(&mut p, db.daten, angelegt_von_, angelegt_am_);
//             }
//             super::mach_geaendert(&mut p, db.daten, geaendert_von_, geaendert_am_);
//             update(db, &p)?;
//         }
//     } else {
//         super::mach_angelegt(&mut p, db.daten, angelegt_von_, angelegt_am_);
//         if !geaendert_von_.is_none() {
//             super::mach_geaendert(&mut p, db.daten, geaendert_von_, geaendert_am_);
//         }
//         insert(db, &p)?;
//     }
//     return Ok(p);
// }

// /// Save dataset without revision columns.
// #[allow(dead_code)]
// pub fn save(
//     db: &mut DbContext,
//     mandant_nr_: &i32,
//     benutzer_id_: &String,
//     passwort_: &Option<String>,
//     berechtigung_: &i32,
//     akt_periode_: &i32,
//     person_nr_: &i32,
//     geburt_: &Option<NaiveDate>
// ) -> Result<Benutzer> {
//     save0(
//         db,
//         mandant_nr_,
//         benutzer_id_,
//         passwort_,
//         berechtigung_,
//         akt_periode_,
//         person_nr_,
//         geburt_,
//         &None,
//         &None,
//         &None,
//         &None,
//     )
// }

/// Get dataset by primary key.
#[allow(dead_code)]
pub fn get(
    conn: &mut SqliteConnection,
    // data: &ServiceData,
    mandant_nr_: &i32,
    benutzer_id_: &String,
) -> Result<Option<Benutzer>> {
    let p = BENUTZER::table
        .filter(
            BENUTZER::mandant_nr
                .eq(mandant_nr_)
                .and(BENUTZER::benutzer_id.eq(benutzer_id_.clone())),
        )
        .first::<Benutzer>(conn)
        .optional()?;
    Ok(p)
}

/// Get dataset by primary key.
pub fn get2(con: &mut SqliteConnection, b: &Benutzer) -> Result<Option<Benutzer>> {
    let p = BENUTZER::table
        .filter(
            BENUTZER::mandant_nr
                .eq(b.mandant_nr)
                .and(BENUTZER::benutzer_id.eq(b.benutzer_id.clone())),
        )
        .first::<Benutzer>(con)
        .optional()?;
    Ok(p)
}

/// Get list.
#[allow(dead_code)]
pub fn get_list(con: &mut SqliteConnection, mandant_nr_: i32) -> Result<Vec<Benutzer>> {
    let list = BENUTZER::table
        .filter(BENUTZER::mandant_nr.eq(mandant_nr_))
        .load::<Benutzer>(con)?;
    Ok(list)
}

/// Insert a dataset.
pub fn insert<'a>(
    con: &mut SqliteConnection,
    data: &mut ServiceData,
    b: &'a Benutzer,
) -> Result<&'a Benutzer> {
    let rows = diesel::insert_into(BENUTZER::table)
        .values(b)
        .execute(con)?;
    if rows <= 0 {
        return Err(ServiceError::NotFound);
    }
    data.ul.add(&UndoEntry::benutzer(None, Some(b)));
    Ok(b)
}

// /// Update a dataset.
// pub fn update<'a>(db: &mut DbContext, b: &'a Benutzer) -> Result<&'a Benutzer> {
//     let oo = get2(&db, b)?;
//     let rows = diesel::update(
//         BENUTZER::table.filter(
//             BENUTZER::mandant_nr.eq(b.mandant_nr)
//             .and(BENUTZER::benutzer_id.eq(b.benutzer_id.clone())),
//         ),
//     )
//     .set((
//         BENUTZER::passwort.eq(b.passwort.as_ref()),
//         BENUTZER::berechtigung.eq(b.berechtigung),
//         BENUTZER::akt_periode.eq(b.akt_periode),
//         BENUTZER::person_nr.eq(b.person_nr),
//         BENUTZER::geburt.eq(b.geburt),
//         BENUTZER::angelegt_von.eq(b.angelegt_von.as_ref()),
//         BENUTZER::angelegt_am.eq(b.angelegt_am),
//         BENUTZER::geaendert_von.eq(b.geaendert_von.as_ref()),
//         BENUTZER::geaendert_am.eq(b.geaendert_am),
//     ))
//     .execute(db.c)?;
//     if rows <= 0 || oo.is_none() {
//         return Err(RsbpError::NotFound);
//     }
//     if let Some(o) = oo {
//         db.ul.add(&UndoEntry::benutzer(Some(&o), Some(b)));
//     }
//     Ok(b)
// }

/// Delete a dataset.
pub fn delete(con: &mut SqliteConnection, data: &mut ServiceData, b: &Benutzer) -> Result<()> {
    let oo = get2(con, b)?;
    let rows = diesel::delete(
        BENUTZER::table.filter(
            BENUTZER::mandant_nr
                .eq(b.mandant_nr)
                .and(BENUTZER::benutzer_id.eq(b.benutzer_id.clone())),
        ),
    )
    .execute(con)?;
    if rows <= 0 || oo.is_none() {
        return Err(ServiceError::NotFound);
    }
    if let Some(o) = oo {
        data.ul.add(&UndoEntry::benutzer(Some(&o), None));
    }
    Ok(())
}
