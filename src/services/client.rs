use crate::base::errors::Result;
use crate::base::service::ServiceDaten;
use crate::services::reps;
use rep::models::Benutzer;

/// Get list with users.
/// * daten: Service data for database access.
/// * returns: List with users.
pub fn get_user_list<'a>(daten: &'a ServiceDaten) -> Result<Vec<Benutzer>> {
    // let c = reps::establish_connection(daten);
    // let db = DbContext::new(daten, &c);
    let l = vec![]; //reps::benutzer::get_list(&db, daten.mandant_nr)?;
    Ok(l)
}
