use crate::base::errors::Result;
use crate::base::service::ServiceData;
use crate::reps;
use basis::functions;
use rep::models::Benutzer;

/// Get list with users.
/// * data: Service data for database access.
/// * returns: List with users.
pub fn get_user_list<'a>(data: &'a mut ServiceData) -> Result<Vec<Benutzer>> {
    // let c = reps::establish_connection(daten);
    // let db = DbContext::new(daten, &c);
    //let l = vec![];
    if functions::mach_nichts() == 0 {
        let l = reps::benutzer::get_list(data.conn, data.mandant_nr)?;
        Ok(l)
    } else {
        Err(crate::base::errors::ServiceError::NotFound)
        // Err(crate::base::errors::ServiceError::DieselError {
        //     source: diesel::result::Error::AlreadyInTransaction,
        // })
    }
}
