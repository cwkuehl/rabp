use crate::base::errors::Result;
use crate::base::service::ServiceData;
use crate::{reps, ServiceError, UndoRedoStack};
use basis::functions;
use diesel::Connection;
use rep::models::Benutzer;

/// Get list with users.
/// * con: Database connection.
/// * data: Service data for database access.
/// * change: Should the user list be changed?
/// * returns: List with users.
pub fn get_user_list<'a>(
    con: &'a mut diesel::SqliteConnection,
    data: &'a mut ServiceData,
    mask_password: bool,
    change: bool,
) -> Result<Vec<Benutzer>> {
    let tr = con.transaction::<Vec<Benutzer>, ServiceError, _>(|e| {
        if functions::mach_nichts() == 0 {
            if change {
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
            }
            let l = reps::benutzer::get_list(e, data.mandant_nr)?;
            if mask_password {
                // Masks passwords.
                let mut ben = Vec::new();
                for b in l {
                    let mut bc = b.clone();
                    bc.passwort = Some("xxx".to_string());
                    ben.push(bc);
                }
                return Ok(ben);
            }
            Ok(l)
        } else {
            Err(crate::base::errors::ServiceError::NotFound)
        }
    });
    tr
}

/// Undoes last transaction.
/// * con: Database connection.
/// * data: Service data for database access and UndoList.
/// * returns: Was something changed?
pub fn undo<'a>(
    con: &'a mut diesel::SqliteConnection,
    data: &'a mut ServiceData,
) -> Result<bool> {
    let tr = con.transaction::<bool, ServiceError, _>(|e| {
        let r = UndoRedoStack::undo(e, data)?; // undolist)?;
        Ok(r)
    });
    tr
}

/// Redoes last transaction.
/// * con: Database connection.
/// * data: Service data for database access and UndoList.
/// * returns: Was something changed?
pub fn redo<'a>(
  con: &'a mut diesel::SqliteConnection,
  data: &'a mut ServiceData,
) -> Result<bool> {
  let tr = con.transaction::<bool, ServiceError, _>(|e| {
      let r = UndoRedoStack::redo(e, data)?;
      Ok(r)
  });
  tr
}
