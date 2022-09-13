use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("config-desc-map.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(
        &mut file,
        "pub static CONFIG_DESCRIPTIONS: phf::Map<&'static str, &'static str> = {}",
        phf_codegen::Map::new()
            .entry(
                "depth",
                "\"Depth to traverse though 'workspace' paths. Defaults to None\""
            )
            .build()
    )
    .unwrap();
    writeln!(&mut file, ";").unwrap();
}
