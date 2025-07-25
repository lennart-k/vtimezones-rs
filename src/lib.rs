include!(concat!(env!("OUT_DIR"), "/timezones.rs"));

pub const IANA_TZDB_VERSION: &str = "2025b";

#[cfg(test)]
mod tests {
    use crate::VTIMEZONES;

    #[test]
    fn test_basic() {
        assert!(VTIMEZONES.get("Europe/Berlin").is_some());
    }

    #[test]
    fn test_symlink() {
        assert!(VTIMEZONES.get("Africa/Bamako").is_some());
    }
}
