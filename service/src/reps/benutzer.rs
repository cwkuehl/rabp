use diesel::QueryResult;
//use diesel::{QueryDsl, QueryResult, RunQueryDsl};
//use crate::reps::DbCon;
//use diesel::result::Error;
use rep::models::Benutzer;
use rep::schema::BENUTZER;

// TODO: async
// https://docs.rs/diesel-async/latest/diesel_async/pooled_connection/deadpool/index.html (not for sqlite)
// pub async fn get_all_async(con: &DbCon) -> Result<Vec<Benutzer>, Error> {
//     let result = con.run(|c| BENUTZER::table.load::<Benutzer>(c)).await;
//     return result;
// }

pub fn get_all(con: &mut diesel::SqliteConnection, mandant_nr_: i32) -> QueryResult<Vec<Benutzer>> {
    // let result = BENUTZER::table.get_results::<Benutzer>(con);
    use diesel::prelude::*;

    let result = BENUTZER::table
        .filter(BENUTZER::mandant_nr.eq(mandant_nr_))
        .load::<Benutzer>(con);
    return result;
}
