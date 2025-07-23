use std::{
    env,
    fs::{File, read_to_string},
    io::Write,
    iter::zip,
    path::Path,
};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let crate_dir = env!("CARGO_MANIFEST_DIR");

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=tzdata_ics");

    let zones_tab = read_to_string("tzdata_ics/zones.tab");
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
        .map(|name| format!("{crate_dir}/tzdata_ics/{name}.ics"))
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
}
