use std::{
    env,
    fs::{File, read_to_string},
    io::Write,
    iter::zip,
    path::Path,
    process::Command,
};

const IANA_TZDB_VERSION: &str = "2025b";

fn main() {
    let out_dir = format!("{}/vtimezones", env::var("OUT_DIR").unwrap());

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=tzdata");
    println!("cargo::rerun-if-changed=vzic");

    assert!(
        Command::new("make")
            .args(["-C", "vzic"])
            .env("OLSON_DIR", "")
            .env(
                "PRODUCT_ID",
                "-//github.com/lennart-k/vzic-rs//RustiCal Calendar server//EN",
            )
            .env("TZID_PREFIX", "")
            .env("CREATE_SYMLINK", "1")
            .status()
            .unwrap()
            .success()
    );

    assert!(
        Command::new("./vzic/vzic")
            .arg("--dump")
            .args(["--olson-dir", "tzdata"])
            .args(["--output-dir", &out_dir])
            .status()
            .unwrap()
            .success()
    );

    let zones_tab = read_to_string(format!("{out_dir}/zones.tab"));
    let zonenames: Vec<&str> = zones_tab
        .as_ref()
        .unwrap()
        .lines()
        .map(|line| {
            if line.contains(' ') {
                line.rsplit_once(' ').unwrap().1
            } else {
                line
            }
        })
        .collect();

    let zonepaths: Vec<_> = zonenames
        .iter()
        .map(|name| format!("{out_dir}/{name}.ics"))
        .collect();

    let mut tzmap = phf_codegen::Map::new();
    for (zonename, zonepath) in zip(zonenames, zonepaths) {
        tzmap.entry(zonename, format!("include_str!(\"{zonepath}\")"));
    }

    let dest_path = Path::new(&out_dir).join("timezones.rs");
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(
        format!(
            "pub static VTIMEZONES: phf::Map<&'static str, &'static str> = {};\n",
            tzmap.build()
        )
        .as_bytes(),
    )
    .unwrap();
    f.write_all(format!("pub const IANA_TZDB_VERSION: &str = \"{IANA_TZDB_VERSION}\";").as_bytes())
        .unwrap();
}
