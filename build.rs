// Copyright (c) 2022 TOYOTA MOTOR CORPORATION. Licensed under MIT OR Apache-2.0.

extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // uses cbindgen.toml
    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file("springql.h");
}
