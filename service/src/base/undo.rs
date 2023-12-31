use super::super::reps;
use super::errors::Result;
use crate::base::service::ServiceData;
use rep::models::{
    AdAdresse, AdPerson, AdSitz, Benutzer, ByteDaten, FzBuch, FzBuchautor, FzBuchserie,
    FzBuchstatus, FzFahrrad, FzFahrradstand, FzNotiz, HhBilanz, HhBuchung, HhEreignis, HhKonto,
    HhPeriode, MaMandant, MaParameter, SbEreignis, SbFamilie, SbKind, SbPerson, SbQuelle, SoKurse,
    TbEintrag, TbEintragOrt, TbOrt, WpAnlage, WpBuchung, WpKonfiguration, WpStand, WpWertpapier,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum UndoEntry {
    AdAdresse { original: String, actual: String },
    AdPerson { original: String, actual: String },
    AdSitz { original: String, actual: String },
    Benutzer { original: String, actual: String },
    ByteDaten { original: String, actual: String },
    FzBuch { original: String, actual: String },
    FzBuchautor { original: String, actual: String },
    FzBuchserie { original: String, actual: String },
    FzBuchstatus { original: String, actual: String },
    FzFahrrad { original: String, actual: String },
    FzFahrradstand { original: String, actual: String },
    FzNotiz { original: String, actual: String },
    HhBilanz { original: String, actual: String },
    HhBuchung { original: String, actual: String },
    HhEreignis { original: String, actual: String },
    HhKonto { original: String, actual: String },
    HhPeriode { original: String, actual: String },
    MaMandant { original: String, actual: String },
    MaParameter { original: String, actual: String },
    SbEreignis { original: String, actual: String },
    SbFamilie { original: String, actual: String },
    SbKind { original: String, actual: String },
    SbPerson { original: String, actual: String },
    SbQuelle { original: String, actual: String },
    SoKurse { original: String, actual: String },
    TbEintrag { original: String, actual: String },
    TbEintragOrt { original: String, actual: String },
    TbOrt { original: String, actual: String },
    WpAnlage { original: String, actual: String },
    WpBuchung { original: String, actual: String },
    WpKonfiguration { original: String, actual: String },
    WpStand { original: String, actual: String },
    WpWertpapier { original: String, actual: String },
}

#[allow(dead_code)]
impl UndoEntry {
    pub fn ad_adresse(original: Option<&AdAdresse>, actual: Option<&AdAdresse>) -> Self {
        UndoEntry::AdAdresse {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn ad_person(original: Option<&AdPerson>, actual: Option<&AdPerson>) -> Self {
        UndoEntry::AdPerson {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn ad_sitz(original: Option<&AdSitz>, actual: Option<&AdSitz>) -> Self {
        UndoEntry::AdSitz {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn benutzer(original: Option<&Benutzer>, actual: Option<&Benutzer>) -> Self {
        UndoEntry::Benutzer {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn byte_daten(original: Option<&ByteDaten>, actual: Option<&ByteDaten>) -> Self {
        UndoEntry::ByteDaten {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn fz_buch(original: Option<&FzBuch>, actual: Option<&FzBuch>) -> Self {
        UndoEntry::FzBuch {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn fz_buchautor(original: Option<&FzBuchautor>, actual: Option<&FzBuchautor>) -> Self {
        UndoEntry::FzBuchautor {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn fz_buchserie(original: Option<&FzBuchserie>, actual: Option<&FzBuchserie>) -> Self {
        UndoEntry::FzBuchserie {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn fz_buchstatus(original: Option<&FzBuchstatus>, actual: Option<&FzBuchstatus>) -> Self {
        UndoEntry::FzBuchstatus {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn fz_fahrrad(original: Option<&FzFahrrad>, actual: Option<&FzFahrrad>) -> Self {
        UndoEntry::FzFahrrad {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn fz_fahrradstand(
        original: Option<&FzFahrradstand>,
        actual: Option<&FzFahrradstand>,
    ) -> Self {
        UndoEntry::FzFahrradstand {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn fz_notiz(original: Option<&FzNotiz>, actual: Option<&FzNotiz>) -> Self {
        UndoEntry::FzNotiz {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn hh_bilanz(original: Option<&HhBilanz>, actual: Option<&HhBilanz>) -> Self {
        UndoEntry::HhBilanz {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn hh_buchung(original: Option<&HhBuchung>, actual: Option<&HhBuchung>) -> Self {
        UndoEntry::HhBuchung {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn hh_ereignis(original: Option<&HhEreignis>, actual: Option<&HhEreignis>) -> Self {
        UndoEntry::HhEreignis {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn hh_konto(original: Option<&HhKonto>, actual: Option<&HhKonto>) -> Self {
        UndoEntry::HhKonto {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn hh_periode(original: Option<&HhPeriode>, actual: Option<&HhPeriode>) -> Self {
        UndoEntry::HhPeriode {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn ma_mandant(original: Option<&MaMandant>, actual: Option<&MaMandant>) -> Self {
        UndoEntry::MaMandant {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn ma_parameter(original: Option<&MaParameter>, actual: Option<&MaParameter>) -> Self {
        UndoEntry::MaParameter {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn sb_ereignis(original: Option<&SbEreignis>, actual: Option<&SbEreignis>) -> Self {
        UndoEntry::SbEreignis {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn sb_familie(original: Option<&SbFamilie>, actual: Option<&SbFamilie>) -> Self {
        UndoEntry::SbFamilie {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn sb_kind(original: Option<&SbKind>, actual: Option<&SbKind>) -> Self {
        UndoEntry::SbKind {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn sb_person(original: Option<&SbPerson>, actual: Option<&SbPerson>) -> Self {
        UndoEntry::SbPerson {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn sb_quelle(original: Option<&SbQuelle>, actual: Option<&SbQuelle>) -> Self {
        UndoEntry::SbQuelle {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn so_kurse(original: Option<&SoKurse>, actual: Option<&SoKurse>) -> Self {
        UndoEntry::SoKurse {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn tb_eintrag(original: Option<&TbEintrag>, actual: Option<&TbEintrag>) -> Self {
        UndoEntry::TbEintrag {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn tb_eintrag_ort(original: Option<&TbEintragOrt>, actual: Option<&TbEintragOrt>) -> Self {
        UndoEntry::TbEintragOrt {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn tb_ort(original: Option<&TbOrt>, actual: Option<&TbOrt>) -> Self {
        UndoEntry::TbOrt {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn wp_anlage(original: Option<&WpAnlage>, actual: Option<&WpAnlage>) -> Self {
        UndoEntry::WpAnlage {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn wp_buchung(original: Option<&WpBuchung>, actual: Option<&WpBuchung>) -> Self {
        UndoEntry::WpBuchung {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn wp_konfiguration(
        original: Option<&WpKonfiguration>,
        actual: Option<&WpKonfiguration>,
    ) -> Self {
        UndoEntry::WpKonfiguration {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn wp_stand(original: Option<&WpStand>, actual: Option<&WpStand>) -> Self {
        UndoEntry::WpStand {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn wp_wertpapier(original: Option<&WpWertpapier>, actual: Option<&WpWertpapier>) -> Self {
        UndoEntry::WpWertpapier {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }

    fn to_string<T>(ser: Option<&T>) -> String
    where
        T: ?Sized + Serialize,
    {
        let mut o = String::new();
        if let Some(e) = ser {
            o = serde_json::to_string(e).unwrap_or(o);
        }
        o
    }

    pub fn from_str<'a, T>(s: &'a String) -> Result<Option<T>>
    where
        T: Deserialize<'a>,
    {
        if s.is_empty() {
            return Ok(None);
        }
        let e: T = serde_json::from_str::<'a, T>(s.as_str()).unwrap();
        Ok(Some(e))
    }
}

#[derive(Debug, Clone)]
pub struct UndoList {
    list: Vec<UndoEntry>,
}

impl UndoList {
    pub fn new() -> Self {
        return UndoList { list: Vec::new() };
    }

    pub fn add(&mut self, e: &UndoEntry) {
        self.list.push(e.clone());
    }

    pub fn is_empty(&self) -> bool {
        self.list.len() <= 0
    }

    pub fn add_list(&mut self, ul: &UndoList) {
        for e in ul.list.iter() {
            self.add(&e);
        }
    }

    // pub fn clone(&self) -> Arc<UndoList> {
    //     let mut aul = UndoList::new();
    //     for e in self.list.iter() {
    //         aul.add(&e.clone());
    //     }
    //     Arc::new(aul)
    // }
}

// lazy_static! {
//     static ref UNDO_STACK: Arc<RwLock<UndoRedoStack>> = Arc::new(RwLock::new(UndoRedoStack::new()));
// }

#[derive(Clone, Debug)]
pub struct UndoRedoStack {
    session_id: String,
    undo: Vec<UndoList>,
    redo: Vec<UndoList>,
}

impl UndoRedoStack {
    pub fn new(session_id: String) -> Self {
        return UndoRedoStack {
            session_id,
            undo: Vec::new(),
            redo: Vec::new(),
        };
    }

    /// Adds UndoList to stack after commit.
    pub fn add_undo(&mut self, ul: &UndoList) {
        if ul.is_empty() {
            return;
        }
        self.undo.push(ul.clone());
        self.redo.clear(); // All Redos are invalid after commit.
        self.trace();
    }

    pub fn get_last_undo(&self) -> UndoList {
        if self.undo.is_empty() {
            return UndoList::new();
        }
        self.undo.last().unwrap().clone()
    }

    pub fn remove_undo(&mut self, ul: &UndoList) {
        if ul.is_empty() {
            return;
        }
        let li = self.undo.len() - 1;
        // self.trace();
        self.undo.remove(li);
        // self.trace();
        self.redo.push(ul.clone());
        self.trace();
    }

    pub fn get_last_redo(&self) -> UndoList {
        if self.redo.is_empty() {
            return UndoList::new();
        }
        self.redo.last().unwrap().clone()
    }

    pub fn remove_redo(&mut self, ul: &UndoList) {
        if ul.is_empty() {
            return;
        }
        let li = self.redo.len() - 1;
        self.redo.remove(li);
        self.undo.push(ul.clone());
        self.trace();
    }

    fn trace(&self) {
        if cfg!(debug_assertions) {
            println!(
                "session_id {}  undo {}  redo {}",
                self.session_id,
                self.undo.len(),
                self.redo.len()
            );
            // for e in self.undo.iter() {
            //     println!("  undo {}", e.list.len());
            // }
            // for e in self.redo.iter() {
            //     println!("  redo {}", e.list.len());
            // }
        }
    }

    /// Undoes last transaction.
    /// * con: Database connection.
    /// * data: Service data for database access.
    /// * undolist: Affected UndoList.
    /// * returns: Was something changed?
    #[allow(unused_variables)]
    pub fn undo<'a>(
        con: &'a mut diesel::SqliteConnection,
        data: &'a mut ServiceData,
        //undolist: Box<UndoList>,
    ) -> Result<bool> {
        // TODO        let mut guard = match UNDO_STACK.write() {
        //             Ok(guard) => guard,
        //             Err(poisoned) => poisoned.into_inner(),
        //         };
        let mut r = false;
        //if let Some(ul) = undolist {
        //let ul = &ul0.clone();
        let undolist = data.ul.clone();
        for e in undolist.list.iter() {
            //for e in data.ul.list.iter() {
            //println!("e: {:?}", e);
            match e {
                UndoEntry::AdAdresse { original, actual } => {
                    // TODO reps::ad_adresse::undo(con, data, original, actual)?;
                }
                UndoEntry::AdPerson { original, actual } => {
                    // reps::ad_person::undo(con, data, original, actual)?;
                }
                UndoEntry::AdSitz { original, actual } => {
                    // reps::ad_sitz::undo(con, data, original, actual)?;
                }
                UndoEntry::Benutzer { original, actual } => {
                    reps::benutzer::undo(con, data, original, actual)?;
                }
                UndoEntry::ByteDaten { original, actual } => {
                    // reps::byte_daten::undo(con, data, original, actual)?;
                }
                UndoEntry::FzBuch { original, actual } => {
                    // reps::fz_buch::undo(con, data, original, actual)?;
                }
                UndoEntry::FzBuchautor { original, actual } => {
                    // reps::fz_buchautor::undo(con, data, original, actual)?;
                }
                UndoEntry::FzBuchserie { original, actual } => {
                    // reps::fz_buchserie::undo(con, data, original, actual)?;
                }
                UndoEntry::FzBuchstatus { original, actual } => {
                    // reps::fz_buchstatus::undo(con, data, original, actual)?;
                }
                UndoEntry::FzFahrrad { original, actual } => {
                    // reps::fz_fahrrad::undo(con, data, original, actual)?;
                }
                UndoEntry::FzFahrradstand { original, actual } => {
                    // reps::fz_fahrradstand::undo(con, data, original, actual)?;
                }
                UndoEntry::FzNotiz { original, actual } => {
                    // reps::fz_notiz::undo(con, data, original, actual)?;
                }
                UndoEntry::HhBilanz { original, actual } => {
                    // reps::hh_bilanz::undo(con, data, original, actual)?;
                }
                UndoEntry::HhBuchung { original, actual } => {
                    // reps::hh_buchung::undo(con, data, original, actual)?;
                }
                UndoEntry::HhEreignis { original, actual } => {
                    // reps::hh_ereignis::undo(con, data, original, actual)?;
                }
                UndoEntry::HhKonto { original, actual } => {
                    // reps::hh_konto::undo(con, data, original, actual)?;
                }
                UndoEntry::HhPeriode { original, actual } => {
                    // reps::hh_periode::undo(con, data, original, actual)?;
                }
                UndoEntry::MaMandant { original, actual } => {
                    // reps::ma_mandant::undo(con, data, original, actual)?;
                }
                UndoEntry::MaParameter { original, actual } => {
                    // reps::ma_parameter::undo(con, data, original, actual)?;
                }
                UndoEntry::SbEreignis { original, actual } => {
                    // reps::sb_ereignis::undo(con, data, original, actual)?;
                }
                UndoEntry::SbFamilie { original, actual } => {
                    // reps::sb_familie::undo(con, data, original, actual)?;
                }
                UndoEntry::SbKind { original, actual } => {
                    // reps::sb_kind::undo(con, data, original, actual)?;
                }
                UndoEntry::SbPerson { original, actual } => {
                    // reps::sb_person::undo(con, data, original, actual)?;
                }
                UndoEntry::SbQuelle { original, actual } => {
                    // reps::sb_quelle::undo(con, data, original, actual)?;
                }
                UndoEntry::SoKurse { original, actual } => {
                    // reps::so_kurse::undo(con, data, original, actual)?;
                }
                UndoEntry::TbEintrag { original, actual } => {
                    reps::tb_eintrag::undo(con, data, original, actual)?;
                }
                UndoEntry::TbEintragOrt { original, actual } => {
                    reps::tb_eintrag_ort::undo(con, data, original, actual)?;
                }
                UndoEntry::TbOrt { original, actual } => {
                    reps::tb_ort::undo(con, data, original, actual)?;
                }
                UndoEntry::WpAnlage { original, actual } => {
                    // reps::wp_anlage::undo(con, data, original, actual)?;
                }
                UndoEntry::WpBuchung { original, actual } => {
                    // reps::wp_buchung::undo(con, data, original, actual)?;
                }
                UndoEntry::WpKonfiguration { original, actual } => {
                    // reps::wp_konfiguration::undo(con, data, original, actual)?;
                }
                UndoEntry::WpStand { original, actual } => {
                    // reps::wp_stand::undo(con, data, original, actual)?;
                }
                UndoEntry::WpWertpapier { original, actual } => {
                    // reps::wp_wertpapier::undo(con, data, original, actual)?;
                }
            };
            r = true;
        }
        data.ul.list.clear();
        data.ul.add_list(&undolist);
        //self.remove_undo(&ul);
        //}
        Ok(r)
    }

    /// Redoes last transaction.
    /// * con: Database connection.
    /// * data: Service data for database access.
    /// * returns: Was something changed?
    #[allow(unused_variables)]
    pub fn redo<'a>(
        con: &'a mut diesel::SqliteConnection,
        data: &'a mut ServiceData,
    ) -> Result<bool> {
        // let mut guard = match UNDO_STACK.write() {
        //     Ok(guard) => guard,
        //     Err(poisoned) => poisoned.into_inner(),
        // };
        let mut r = false;
        //if let Some(ul) = undolist {
        //let ul = &ul0.clone();
        let undolist = data.ul.clone();
        for e in undolist.list.iter() {
            //println!("e: {:?}", e);
            match e {
                UndoEntry::AdAdresse { original, actual } => {
                    // TODO reps::ad_adresse::redo(con, data, original, actual)?;
                }
                UndoEntry::AdPerson { original, actual } => {
                    // reps::ad_person::redo(con, data, original, actual)?;
                }
                UndoEntry::AdSitz { original, actual } => {
                    // reps::ad_sitz::redo(con, data, original, actual)?;
                }
                UndoEntry::Benutzer { original, actual } => {
                    reps::benutzer::redo(con, data, original, actual)?;
                }
                UndoEntry::ByteDaten { original, actual } => {
                    // reps::byte_daten::redo(con, data, original, actual)?;
                }
                UndoEntry::FzBuch { original, actual } => {
                    // reps::fz_buch::redo(con, data, original, actual)?;
                }
                UndoEntry::FzBuchautor { original, actual } => {
                    // reps::fz_buchautor::redo(con, data, original, actual)?;
                }
                UndoEntry::FzBuchserie { original, actual } => {
                    // reps::fz_buchserie::redo(con, data, original, actual)?;
                }
                UndoEntry::FzBuchstatus { original, actual } => {
                    // reps::fz_buchstatus::redo(con, data, original, actual)?;
                }
                UndoEntry::FzFahrrad { original, actual } => {
                    // reps::fz_fahrrad::redo(con, data, original, actual)?;
                }
                UndoEntry::FzFahrradstand { original, actual } => {
                    // reps::fz_fahrradstand::redo(con, data, original, actual)?;
                }
                UndoEntry::FzNotiz { original, actual } => {
                    // reps::fz_notiz::redo(con, data, original, actual)?;
                }
                UndoEntry::HhBilanz { original, actual } => {
                    // reps::hh_bilanz::redo(con, data, original, actual)?;
                }
                UndoEntry::HhBuchung { original, actual } => {
                    // reps::hh_buchung::redo(con, data, original, actual)?;
                }
                UndoEntry::HhEreignis { original, actual } => {
                    // reps::hh_ereignis::redo(con, data, original, actual)?;
                }
                UndoEntry::HhKonto { original, actual } => {
                    // reps::hh_konto::redo(con, data, original, actual)?;
                }
                UndoEntry::HhPeriode { original, actual } => {
                    // reps::hh_periode::redo(con, data, original, actual)?;
                }
                UndoEntry::MaMandant { original, actual } => {
                    // reps::ma_mandant::redo(con, data, original, actual)?;
                }
                UndoEntry::MaParameter { original, actual } => {
                    // reps::ma_parameter::redo(con, data, original, actual)?;
                }
                UndoEntry::SbEreignis { original, actual } => {
                    // reps::sb_ereignis::redo(con, data, original, actual)?;
                }
                UndoEntry::SbFamilie { original, actual } => {
                    // reps::sb_familie::redo(con, data, original, actual)?;
                }
                UndoEntry::SbKind { original, actual } => {
                    // reps::sb_kind::redo(con, data, original, actual)?;
                }
                UndoEntry::SbPerson { original, actual } => {
                    // reps::sb_person::redo(con, data, original, actual)?;
                }
                UndoEntry::SbQuelle { original, actual } => {
                    // reps::sb_quelle::redo(con, data, original, actual)?;
                }
                UndoEntry::SoKurse { original, actual } => {
                    // reps::so_kurse::redo(con, data, original, actual)?;
                }
                UndoEntry::TbEintrag { original, actual } => {
                    reps::tb_eintrag::redo(con, data, original, actual)?;
                }
                UndoEntry::TbEintragOrt { original, actual } => {
                    reps::tb_eintrag_ort::redo(con, data, original, actual)?;
                }
                UndoEntry::TbOrt { original, actual } => {
                    reps::tb_ort::redo(con, data, original, actual)?;
                }
                UndoEntry::WpAnlage { original, actual } => {
                    // reps::wp_anlage::redo(con, data, original, actual)?;
                }
                UndoEntry::WpBuchung { original, actual } => {
                    // reps::wp_buchung::redo(con, data, original, actual)?;
                }
                UndoEntry::WpKonfiguration { original, actual } => {
                    // reps::wp_konfiguration::redo(con, data, original, actual)?;
                }
                UndoEntry::WpStand { original, actual } => {
                    // reps::wp_stand::redo(con, data, original, actual)?;
                }
                UndoEntry::WpWertpapier { original, actual } => {
                    // reps::wp_wertpapier::redo(con, data, original, actual)?;
                }
            };
            r = true;
        }
        data.ul.list.clear();
        data.ul.add_list(&undolist);
        // self.remove_redo(&ul);
        // }
        Ok(r)
    }
}
