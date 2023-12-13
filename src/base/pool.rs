use diesel::{
    r2d2::{ConnectionManager, Pool},
    sqlite::SqliteConnection,
};
use std::collections::HashMap;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type UndoMap = HashMap<String, service::UndoRedoStack>;

pub struct UndoPool {
    pub map: UndoMap,
}

impl UndoPool {
    pub fn new() -> UndoPool {
        UndoPool {
            map: HashMap::<String, service::UndoRedoStack>::new(),
        }
    }
}
