use chrono::{Datelike, NaiveDate};
use lazy_static::lazy_static;

/// The function does nothing and always returns 0.
pub fn mach_nichts() -> i32 {
    0
}

/// Converts string to i32.
/// * s: Affected string.
pub fn to_i32(s: &str) -> i32 {
    let x = s.parse::<i32>();
    if let Ok(i) = x {
        return i;
    }
    0
}

/// Converts string to float.
/// * s: Affected string.
pub fn to_f32(s: &str) -> f32 {
    let x = s.parse::<f32>();
    if let Ok(i) = x {
        return i;
    }
    0_f32
}

/// Returns string depending on boolean value.
pub fn iif<'a>(b: bool, strue: &'a str, sfalse: &'a str) -> &'a str {
    if b {
        return strue;
    }
    sfalse
}

/// Cuts first 5 characters if possible.
pub fn m5(s: &str, cut: bool) -> &str {
    if !cut || s.is_empty() || s.len() < 5 {
        s
    } else {
        &s[5..]
    }
}

/// Returns GUID.
pub fn get_uid() -> String {
    let guid = uuid::Uuid::new_v4();
    guid.to_string()
    //format!("{}", guid)
}

lazy_static! {
    static ref MIN_YEAR: i32 = NaiveDate::MIN.year();
    static ref MAX_YEAR: i32 = NaiveDate::MAX.year();
}

/// Gets last day of month.
/// * year: Affected year.
/// * month: Affected one-based month.
fn last_day_of_month(year: i32, month: u32) -> u32 {
    let plus_year = (month / 12) as i32;
    let m = (month % 12) + 1;
    if year > 0 && *MAX_YEAR - year - plus_year < 0 {
        // preventing overflow
        return NaiveDate::from_ymd_opt(*MAX_YEAR, m, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .day();
    }
    let year = year + plus_year;
    if year < *MIN_YEAR {
        return NaiveDate::from_ymd_opt(*MIN_YEAR, m, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .day();
    }
    let ml = NaiveDate::from_ymd_opt(year, m, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
        .day();
    ml
}

/// Adds days, month and years to date.
/// * nd: Affected date.
/// * returns: Added date.
pub fn nd_add_dmy(nd: &NaiveDate, days: i32, months: i32, years: i32) -> Option<NaiveDate> {
    if let Some(d2) = NaiveDate::from_num_days_from_ce_opt(nd.num_days_from_ce() + days) {
        let mut d = d2.day();
        let mut m = d2.month() as i32 + months;
        let mut y = d2.year() + years;
        while m > 12 {
            m -= 12;
            y += 1;
        }
        while m < 1 {
            m += 12;
            y -= 1;
        }
        let ml = last_day_of_month(y, m as u32);
        if d > ml {
            m += 1;
            if m > 12 {
                y += 1;
            }
            d -= ml;
        }
        let d3 = NaiveDate::from_ymd_opt(y, m as u32, d as u32);
        return d3;
    }
    None
}

/// Gets minimum of two dates.
pub fn min_date(d1: &NaiveDate, d2: &NaiveDate) -> NaiveDate {
    let m = match d1 < d2 {
        true => d1,
        _ => d2,
    };
    m.clone()
}

/// Gets maximum of two dates.
pub fn max_date(d1: &NaiveDate, d2: &NaiveDate) -> NaiveDate {
    let m = match d1 > d2 {
        true => d1,
        _ => d2,
    };
    m.clone()
}

/// Converts date string of format YYYY-MM-DD to date.
pub fn to_date(s: &str, default: &NaiveDate) -> NaiveDate {
    match chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        Ok(date) => return date,
        _ => return default.clone(),
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn get_uid() {
    //     assert_eq!(36, super::get_uid().len());
    // }
    #[test]
    fn mach_nichts() {
        assert_eq!(0, super::mach_nichts());
    }
    #[test]
    fn to_i32() {
        assert_eq!(0, super::to_i32(""));
        assert_eq!(0, super::to_i32("x"));
        assert_eq!(1, super::to_i32("1"));
        assert_eq!(-1, super::to_i32("-1"));
    }

    #[test]
    fn to_f32() {
        assert_eq!(0_f32, super::to_f32(""));
        assert_eq!(0_f32, super::to_f32("x"));
        assert_eq!(1_f32, super::to_f32("1"));
        assert_eq!(-1_f32, super::to_f32("-1"));
        assert_eq!(1.1_f32, super::to_f32("1.1"));
        assert_eq!(1.01_f32, super::to_f32("1.01"));
    }

    #[test]
    fn m5() {
        assert_eq!("", super::m5("", true));
        assert_eq!("1", super::m5("1", true));
        assert_eq!("12", super::m5("12", true));
        assert_eq!("123", super::m5("123", true));
        assert_eq!("1234", super::m5("1234", true));
        assert_eq!("", super::m5("12345", true));
        assert_eq!("6", super::m5("123456", true));
        assert_eq!("67", super::m5("1234567", true));
        assert_eq!("", super::m5("", false));
        assert_eq!("1", super::m5("1", false));
        assert_eq!("12", super::m5("12", false));
        assert_eq!("123", super::m5("123", false));
        assert_eq!("1234", super::m5("1234", false));
        assert_eq!("12345", super::m5("12345", false));
        assert_eq!("123456", super::m5("123456", false));
        assert_eq!("1234567", super::m5("1234567", false));
    }
}
