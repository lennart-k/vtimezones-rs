use std::{env, fs::File, io::Write, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let crate_dir = env!("CARGO_MANIFEST_DIR");

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=tzdata_ics");

    let mut tzmap = phf_codegen::Map::new();
    for entry in glob::glob("tzdata_ics/**/*.ics").unwrap() {
        let entry = entry.unwrap();
        let zonename = entry
            .strip_prefix("tzdata_ics/")
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".ics")
            .unwrap();
        let abspath = Path::new(crate_dir).join(entry.as_path());
        let zonepath = abspath.to_str().unwrap();
        tzmap.entry(zonename.to_owned(), format!("include_str!(\"{zonepath}\")"));
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
}
