# vtimezones-rs

Provides timezones from the IANA TZDB as VTIMEZONES for the iCalendar format.

Uses the timezone data from <https://github.com/eggert/tz/> and <https://github.com/libical/vzic> for conversion.

Exports

```rs
pub static VTIMEZONES: phf::Map<&'static str, &'static str>;
pub const IANA_TZDB_VERSION: &str = "2025b";
```

This project is not affiliated with any of its dependencies.
