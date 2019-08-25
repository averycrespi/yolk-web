use built;

use std::env;
use std::path;

fn main() {
    let mut options = built::Options::default();
    options.set_dependencies(true);
    let src = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest = path::Path::new(&env::var("OUT_DIR").unwrap()).join("built.rs");
    built::write_built_file_with_opts(&options, &src, &dest)
        .expect("Failed to acquire build-time information");
}
