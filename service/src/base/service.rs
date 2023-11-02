//use crate::{config::RsbpConfig, res};
use super::enums::RabpLocale;
use chrono::{DateTime, Local, NaiveDate, Timelike};

//#[derive(Debug, Clone)] // TODO derive_debug
pub struct ServiceData<'a> {
    pub conn: &'a mut diesel::SqliteConnection,
    pub mandant_nr: i32,
    pub benutzer_id: String,
    pub heute: NaiveDate,
    pub jetzt: DateTime<Local>,
    pub locale: RabpLocale,
    // TODO UndoList
}

impl<'a> ServiceData<'a> {
    pub fn new(conn: &'a mut diesel::SqliteConnection, mandant_nr: i32, benutzer_id: &str) -> Self {
        let mut now: DateTime<Local> = Local::now();
        now = now.with_nanosecond(0).unwrap_or(now); // nur sekundengenau
        ServiceData {
            conn,
            mandant_nr,
            benutzer_id: String::from(benutzer_id),
            heute: now.date_naive(),
            jetzt: now,
            locale: RabpLocale::De, // TODO from request
        }
    }

    // pub fn new() -> Self {
    //     let daten = get_daten();
    //     daten
    // }
}

// /// Connection of ServiceDaten, database connection and UndoList for fewer parameters.
// pub struct DbContext<'a> {
//     pub daten: &'a ServiceDaten<'a>,
//     pub c: &'a SqliteConnection,
//     pub ul: UndoList,
// }

// impl<'a> DbContext<'a> {
//     /// Initialisierung des Datenbank-Kontextes.
//     /// * daten: Betroffene Service-Daten.
//     /// * c: Betroffene Datenbank-Verbindung.
//     pub fn new(daten: &'a ServiceDaten, c: &'a SqliteConnection) -> Self {
//         DbContext {
//             daten,
//             c,
//             ul: UndoList::new(),
//         }
//     }
// }
