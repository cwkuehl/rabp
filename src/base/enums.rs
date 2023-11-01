/// Locale.
#[derive(Clone, Copy, Debug)]
pub enum RabpLocale {
    /// Deutsch
    De,
    /// English
    En,
}

/// Direction of serching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchDirectionEnum {
    /// no direction
    None = -1,
    /// first entry
    First = 0,
    /// previous entry
    Back = 1,
    /// next entry
    Forward = 2,
    /// last entry
    Last = 3,
}
