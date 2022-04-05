// This file is part of https://github.com/SpringQL/SpringQL which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // uses cbindgen.toml
    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file("springql.h");
}
