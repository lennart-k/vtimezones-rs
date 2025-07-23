include!(concat!(env!("OUT_DIR"), "/vtimezones/timezones.rs"));

#[cfg(test)]
mod tests {
    use std::path::{self, Path};

    use crate::VTIMEZONES;

    #[test]
    fn test_basic() {
        assert!(VTIMEZONES.get("Europe/Berlin").is_some());
    }
}
