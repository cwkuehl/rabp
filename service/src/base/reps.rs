use super::service::ServiceData;
use chrono::NaiveDateTime;
use rep::revision::Revision;

/// Time in milliseconds for changing interval.
const AEND_ZEIT: i64 = 60_000;

pub fn mach_angelegt(
    e: &mut dyn Revision,
    data: &ServiceData,
    von: &Option<String>,
    am: &Option<NaiveDateTime>,
) {
    if von.is_none() {
        e.set_angelegt_von(&Some(data.benutzer_id.clone()));
        e.set_angelegt_am(&Some(data.get_now()));
    } else {
        e.set_angelegt_von(von);
        e.set_angelegt_am(am);
    }
}

pub fn mach_geaendert(
    e: &mut dyn Revision,
    data: &ServiceData,
    von: &Option<String>,
    am: &Option<NaiveDateTime>,
) {
    if von.is_none() {
        let mut datum: Option<NaiveDateTime> = e.get_geaendert_am();
        if datum.is_none() {
            datum = e.get_angelegt_am();
        }
        let mut dauer = AEND_ZEIT + 1;
        let jetzt = data.get_now();
        if let Some(d) = datum {
            // println!("Jetzt: {}  Datum: {}", jetzt, d);
            dauer = jetzt.timestamp_millis() - d.timestamp_millis();
        }
        if datum.is_none() || dauer > AEND_ZEIT {
            e.set_geaendert_von(&Some(data.benutzer_id.clone()));
            e.set_geaendert_am(&Some(jetzt));
        }
    } else {
        e.set_geaendert_von(von);
        e.set_geaendert_am(am);
    }
}
