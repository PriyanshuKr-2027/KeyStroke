use nlprule_build::BinaryBuilder;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    BinaryBuilder::new(&["en"], &out_dir)
        .fallback_to_build_dir(true)
        .build()
        .validate()
        .compile();
}
