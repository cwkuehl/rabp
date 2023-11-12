use super::enums::RabpLocale;
use super::undo::UndoList;
use chrono::{DateTime, Local, NaiveDate, Timelike};
use derive_debug::Dbg;

#[derive(Dbg)]
pub struct ServiceData {
    // <'a> {
    //#[dbg(placeholder = "...")]
    //pub conn: &'a mut diesel::SqliteConnection,
    pub mandant_nr: i32,
    pub benutzer_id: String,
    pub heute: NaiveDate,
    pub jetzt: DateTime<Local>,
    pub locale: RabpLocale,
    pub ul: UndoList,
}

// impl<'a> ServiceData<'a> {
impl ServiceData {
    // pub fn new(conn: &'a mut diesel::SqliteConnection, mandant_nr: i32, benutzer_id: &str) -> Self {
    pub fn new(mandant_nr: i32, benutzer_id: &str) -> Self {
        let mut now: DateTime<Local> = Local::now();
        now = now.with_nanosecond(0).unwrap_or(now); // nur sekundengenau
        ServiceData {
            // conn,
            mandant_nr,
            benutzer_id: String::from(benutzer_id),
            heute: now.date_naive(),
            jetzt: now,
            locale: RabpLocale::De, // TODO from request
            ul: UndoList::new(),
        }
    }

    // pub fn new() -> Self {
    //     let daten = get_daten();
    //     daten
    // }
}
