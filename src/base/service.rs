//use crate::{config::RsbpConfig, res};
use chrono::{DateTime, Local, NaiveDate, Timelike};

#[derive(Debug, Clone)]
pub struct ServiceDaten {
    pub mandant_nr: i32,
    pub benutzer_id: String,
    pub heute: NaiveDate,
    pub jetzt: DateTime<Local>,
    //pub config: RsbpConfig,
}

impl ServiceDaten {
    pub fn new0(mandant_nr: i32, benutzer_id: &str) -> Self {
        let mut now: DateTime<Local> = Local::now();
        now = now.with_nanosecond(0).unwrap_or(now); // nur sekundengenau
        ServiceDaten {
            mandant_nr,
            benutzer_id: String::from(benutzer_id),
            heute: now.date_naive(),
            jetzt: now,
            //config: config.clone(),
        }
    }

    // pub fn new() -> Self {
    //     let daten = get_daten();
    //     daten
    // }
}
