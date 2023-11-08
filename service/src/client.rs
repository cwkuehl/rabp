use crate::base::errors::Result;
use crate::base::service::ServiceData;
use crate::{reps, ServiceError};
use basis::functions;
use diesel::Connection;
use rep::models::Benutzer;

/// Get list with users.
/// * data: Service data for database access.
/// * returns: List with users.
pub fn get_user_list<'a>(data: &'a mut ServiceData) -> Result<Vec<Benutzer>> {
    let tr = data
        .conn
        .transaction::<Vec<Benutzer>, ServiceError, _>(|e| {
            if functions::mach_nichts() == 0 {
                let l = reps::benutzer::get_list(e, data.mandant_nr)?;
                Ok(l)
            } else {
                Err(crate::base::errors::ServiceError::NotFound)
                // Err(crate::base::errors::ServiceError::DieselError {
                //     source: diesel::result::Error::AlreadyInTransaction,
                // })
            }
        });
    tr
}
