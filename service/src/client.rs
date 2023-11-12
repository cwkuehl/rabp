use crate::base::errors::Result;
use crate::base::service::ServiceData;
use crate::{reps, ServiceError};
use basis::functions;
use diesel::Connection;
use rep::models::Benutzer;

/// Get list with users.
/// * data: Service data for database access.
/// * returns: List with users.
pub fn get_user_list<'a>(
    con: &'a mut diesel::SqliteConnection,
    data: &'a mut ServiceData,
) -> Result<Vec<Benutzer>> {
    let tr = con.transaction::<Vec<Benutzer>, ServiceError, _>(|e| {
        if functions::mach_nichts() == 0 {
            let ob = reps::benutzer::get(e, &data.mandant_nr, &"xxx".to_string())?;
            if let Some(b) = ob {
                reps::benutzer::delete(e, data, &b)?;
            } else {
                let b = Benutzer {
                    mandant_nr: data.mandant_nr,
                    benutzer_id: "xxx".to_string(),
                    passwort: None,
                    berechtigung: 0,
                    akt_periode: 0,
                    person_nr: 9999,
                    geburt: None,
                    angelegt_von: Some(data.benutzer_id.to_string()),
                    angelegt_am: Some(data.jetzt.naive_local()),
                    geaendert_von: None,
                    geaendert_am: None,
                };
                reps::benutzer::insert(e, data, &b)?;
            }
            let l = reps::benutzer::get_list(e, data.mandant_nr)?;
            Ok(l)
        } else {
            Err(crate::base::errors::ServiceError::NotFound)
        }
    });
    tr
}
