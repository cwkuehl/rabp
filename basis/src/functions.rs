/// The function does nothing and always returns 0.
pub fn mach_nichts() -> i32 {
    0
}

/// Convert string to i32.
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
