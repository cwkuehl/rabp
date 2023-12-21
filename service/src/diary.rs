use crate::{
    base::{enums::RabpLocale, errors::Result, service::ServiceData},
    reps, ServiceError,
};
use basis::{functions, messages::M};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::Connection;
use rep::{
    models::{TbEintrag, TbOrt},
    models_ext::TbEintragOrtExt,
};

/// Gets last diary entries.
/// * con: Database connection.
/// * data: Service data for database access.
/// * returns: Diary entries or possibly errors.
pub fn get_last_entries<'a>(
    con: &'a mut diesel::SqliteConnection,
    data: &'a mut ServiceData,
) -> Result<Vec<TbEintrag>> {
    let e = reps::tb_eintrag::get_list2(con, data.mandant_nr, 4)?;
    Ok(e)
}

/// Gets a diary entry.
/// * con: Database connection.
/// * data: Service data for database access.
/// * date: Affected date.
/// * returns: Diary entry or possibly errors.
pub fn get_entry<'a>(
    con: &'a mut diesel::SqliteConnection,
    data: &'a mut ServiceData,
    date: &NaiveDate,
) -> Result<Option<TbEintrag>> {
    let e = reps::tb_eintrag::get(con, &data.mandant_nr, date)?;
    Ok(e)
}

/// Gets a position list for a date.
/// * con: Database connection.
/// * data: Service data for database access.
/// * date: Affected date.
/// * returns: Position list or possibly errors.
pub fn get_entry_position_list<'a>(
    con: &'a mut diesel::SqliteConnection,
    data: &'a mut ServiceData,
    date: &NaiveDate,
) -> Result<Option<Vec<TbEintragOrtExt>>> {
    let e = reps::tb_eintrag_ort::get_list_ext2(con, data, date)?;
    Ok(Some(e))
}

/// Saves a diary entry.
/// * con: Database connection.
/// * data: Service data for database access.
/// * date: Affected date.
/// * entry: Affected text entry.
/// * pos: Affected position list.
/// * returns: Possibly errors.
pub fn save_entry<'a>(
    con: &'a mut diesel::SqliteConnection,
    data: &'a mut ServiceData,
    date: &NaiveDate,
    entry: &String,
    pos: &Vec<TbEintragOrtExt>,
) -> Result<()> {
    let e = entry.trim();
    let empty = e.is_empty();
    let tr = con.transaction::<(), ServiceError, _>(|con| {
        let mandant_nr = data.mandant_nr;
        if let Some(ref mut tb) = reps::tb_eintrag::get(con, &data.mandant_nr, date)? {
            if empty {
                // Delete empty entry and leave positions.
                reps::tb_eintrag::delete(con, data, tb)?;
            } else if e != tb.eintrag {
                if tb.replikation_uid.is_none() {
                    tb.replikation_uid = Some(functions::get_uid());
                }
                tb.eintrag = e.to_string();
                tb.geaendert_am = Some(data.get_now());
                tb.geaendert_von = Some(data.benutzer_id.to_string());
                reps::tb_eintrag::update(con, data, tb)?;
            }
        } else if !empty {
            let tb = TbEintrag {
                mandant_nr: mandant_nr,
                datum: date.clone(),
                eintrag: e.to_string(),
                angelegt_am: Some(data.get_now()),
                angelegt_von: Some(data.benutzer_id.to_string()),
                geaendert_am: None,
                geaendert_von: None,
                replikation_uid: Some(functions::get_uid()),
            };
            reps::tb_eintrag::insert(con, data, &tb)?;
        }
        // Saves positions.
        // bestehende Orte lesen
        let mut liste = reps::tb_eintrag_ort::get_list_ext(con, data, Some(date), &0, None, None)?;
        for i in pos {
            let puid = i.ort_uid.clone();
            let mut from = i.datum_von;
            let mut to = i.datum_bis;
            if to < from {
                to = from;
            }
            if *date < from || *date > to {
                from = *date;
                to = *date;
            }
            let listep = reps::tb_eintrag_ort::get_list_ext(
                con,
                data,
                Some(&from),
                &0,
                Some(&to),
                Some(&puid),
            )?;
            let ovop = listep.first();
            if let Some(p) = liste.iter().position(|a| a.ort_uid == puid) {
                liste.remove(p); // nicht mehr löschen
            }
            if listep.is_empty() || ovop.is_none() {
                // Zeitraum leer
                optimize_positions(con, data, &puid, &from, &to, &None, &None)?;
            } else if let Some(vop) = ovop {
                if listep.len() == 1 {
                    if vop.datum_von == from && vop.datum_bis == to {
                        functions::mach_nichts();
                    } else if vop.datum_von <= from && vop.datum_bis >= to {
                        if from == to {
                            functions::mach_nichts(); // Fall: Aus Versehen gelöscht und wieder hinzugefügt.
                        } else {
                            // Zeitraum wird verkürzt.
                            reps::tb_eintrag_ort::save0(
                                con,
                                data,
                                &mandant_nr,
                                &puid,
                                &from,
                                &to,
                                &vop.angelegt_von,
                                &vop.angelegt_am,
                                &None,
                                &None,
                            )?;
                            reps::tb_eintrag_ort::delete(con, data, vop)?;
                        }
                    } else {
                        // Nicht verkürzen.
                        let mfrom = functions::min_date(&vop.datum_von, &from);
                        let mto = functions::max_date(&vop.datum_bis, &to);
                        if !(vop.datum_von == mfrom && vop.datum_bis == mto) {
                            // Maximaler Zeitraum
                            optimize_positions(
                                con,
                                data,
                                &puid,
                                &mfrom,
                                &mto,
                                &vop.angelegt_von,
                                &vop.angelegt_am,
                            )?;
                            reps::tb_eintrag_ort::delete(con, data, vop)?;
                        }
                    }
                } else {
                    // listep.Count >= 1
                    let mut mfrom = from;
                    let mut mto = to;
                    for p in listep.iter() {
                        if p.datum_von < mfrom {
                            mfrom = p.datum_von;
                        }
                        if p.datum_bis > mto {
                            mto = p.datum_bis;
                        }
                        reps::tb_eintrag_ort::delete(con, data, p)?;
                    }
                    // Maximaler Zeitraum
                    optimize_positions(
                        con,
                        data,
                        &puid,
                        &mfrom,
                        &mto,
                        &vop.angelegt_von,
                        &vop.angelegt_am,
                    )?;
                }
            }
        }
        // überflüssige Orte löschen.
        for vo in liste {
            if vo.datum_von == vo.datum_bis {
                reps::tb_eintrag_ort::delete(con, data, &vo)?; // Eintrag löschen
            } else if vo.datum_von == *date {
                // Einen Tag vorne verkürzen
                if let Some(d) = functions::nd_add_dmy(date, 1, 0, 0) {
                    reps::tb_eintrag_ort::save0(
                        con,
                        data,
                        &mandant_nr,
                        &vo.ort_uid,
                        &d,
                        &vo.datum_bis,
                        &vo.angelegt_von,
                        &vo.angelegt_am,
                        &None,
                        &None,
                    )?;
                    reps::tb_eintrag_ort::delete(con, data, &vo)?;
                }
            } else if vo.datum_bis == *date {
                // Einen Tag hinten verkürzen
                if let Some(d) = functions::nd_add_dmy(date, -1, 0, 0) {
                    reps::tb_eintrag_ort::save0(
                        con,
                        data,
                        &mandant_nr,
                        &vo.ort_uid,
                        &vo.datum_von,
                        &d,
                        &vo.angelegt_von,
                        &vo.angelegt_am,
                        &None,
                        &None,
                    )?;
                    reps::tb_eintrag_ort::delete(con, data, &vo)?;
                }
            } else {
                // Einen Tag herausschneiden
                if let (Some(dp), Some(dm)) = (
                    functions::nd_add_dmy(date, 1, 0, 0),
                    functions::nd_add_dmy(date, -1, 0, 0),
                ) {
                    reps::tb_eintrag_ort::save0(
                        con,
                        data,
                        &mandant_nr,
                        &vo.ort_uid,
                        &vo.datum_von,
                        &dm,
                        &vo.angelegt_von,
                        &vo.angelegt_am,
                        &None,
                        &None,
                    )?;
                    reps::tb_eintrag_ort::save0(
                        con,
                        data,
                        &mandant_nr,
                        &vo.ort_uid,
                        &dp,
                        &vo.datum_bis,
                        &vo.angelegt_von,
                        &vo.angelegt_am,
                        &None,
                        &None,
                    )?;
                    reps::tb_eintrag_ort::delete(con, data, &vo)?;
                }
            }
        }
        Ok(())
    });
    tr
}

/// Optimieren der Positionen, d.h. verlängern oder Lücke füllen.
/// * con: Database connection.
/// * data: Service data for database access.
/// * puid: Affected position ID.
/// * from: Affected from date.
/// * to: Affected to date.
/// * created_by: Affected creation user id.
/// * created_at: Affection creation time.
fn optimize_positions<'a>(
    con: &'a mut diesel::SqliteConnection,
    data: &'a mut ServiceData,
    puid: &String,
    from: &NaiveDate,
    to: &NaiveDate,
    created_by: &Option<String>,
    created_at: &Option<NaiveDateTime>,
) -> Result<()> {
    let listeb =
        reps::tb_eintrag_ort::get_list_ext(con, data, Some(&from), &-1, None, Some(&puid))?;
    let listea = reps::tb_eintrag_ort::get_list_ext(con, data, Some(&to), &1, None, Some(&puid))?;
    let obef = listeb.first();
    let oaft = listea.first();
    let mandant_nr = data.mandant_nr;

    if let Some(bef) = obef {
        if let Some(aft) = oaft {
            // Lücke füllen
            reps::tb_eintrag_ort::save0(
                con,
                data,
                &mandant_nr,
                &puid,
                &bef.datum_von,
                &aft.datum_bis,
                &bef.angelegt_von,
                &bef.angelegt_am,
                &None,
                &None,
            )?;
            reps::tb_eintrag_ort::delete(con, data, bef)?;
            reps::tb_eintrag_ort::delete(con, data, aft)?;
        } else {
            // Zeitraum hinten anhängen
            reps::tb_eintrag_ort::save0(
                con,
                data,
                &mandant_nr,
                &puid,
                &bef.datum_von,
                to,
                &bef.angelegt_von,
                &bef.angelegt_am,
                &None,
                &None,
            )?;
            reps::tb_eintrag_ort::delete(con, data, bef)?;
        }
    } else if let Some(aft) = oaft {
        // Zeitraum vorne anhängen
        reps::tb_eintrag_ort::save0(
            con,
            data,
            &mandant_nr,
            &puid,
            from,
            &aft.datum_bis,
            &aft.angelegt_von,
            &aft.angelegt_am,
            &None,
            &None,
        )?;
        reps::tb_eintrag_ort::delete(con, data, aft)?;
    } else {
        // Neu
        reps::tb_eintrag_ort::save0(
            con,
            data,
            &mandant_nr,
            &puid,
            from,
            to,
            created_by,
            created_at,
            &None,
            &None,
        )?;
    }
    Ok(())
}

/// Deletes a position.
/// * con: Database connection.
/// * data: Service data for database access.
/// * e: Affected Entity.
/// * returns: Possibly errors.
pub fn delete_position<'a>(
    con: &'a mut diesel::SqliteConnection,
    data: &'a mut ServiceData,
    e: &TbOrt,
) -> Result<()> {
    let tr = con.transaction::<(), ServiceError, _>(|con| {
        let plist = reps::tb_eintrag_ort::get_list_ext(con, data, None, &0, None, Some(&e.uid))?;
        if let Some(p) = plist.first() {
            return Err(ServiceError::error_string(
                M::tb013(&p.datum_von, data.locale == RabpLocale::De).as_str(),
            ));
        }
        reps::tb_ort::delete(con, data, e)?;
        Ok(())
    });
    tr
}
