use crate::reps::DbCon;
use rep::models::Benutzer;
use diesel::result::Error;
use rep::schema::BENUTZER;
use crate::diesel::RunQueryDsl;

pub async fn get_all(con: &DbCon) -> Result<Vec<Benutzer>, Error> {
  let result = con.run(|c| {
    BENUTZER::table
          .load::<Benutzer>(c)
  }).await;
  return result;
}

