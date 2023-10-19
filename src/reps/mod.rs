pub mod benutzer;

use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel::SqliteConnection;

#[database("sqlite_db")]
pub struct DbCon(SqliteConnection);
