# vtimezones-rs

Provides timezones from the IANA TZDB as VTIMEZONES for the iCalendar format.

Uses the timezone data from <https://github.com/eggert/tz/> and <https://github.com/libical/vzic> for conversion.

Exports

```rs
pub static VTIMEZONES: phf::Map<&'static str, &'static str>;
pub const IANA_TZDB_VERSION: &str = "2025b";
```

This project is not affiliated with any of its dependencies.

## Building

- Run `make` to compile the timezones to ics
- Run `cargo build`

The compilation is not included in `build.rs` because then `gtk` (which is a vzic dependency) would need to be available on any machine wanting to compile this crate.

## Updating TZDB

- `git -C tzdata checkout <new version>`
- Update `IANA_TZDB_VERSION` in `src/lib.rs`
- Update `IANA_TZDB_VERSION` in `README.md`
- Run `make`
