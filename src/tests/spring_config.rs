// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use std::ffi::CString;

use crate::*;

#[test]
fn test_spring_config_default() {
    unsafe {
        let config = spring_config_default();
        assert!(!config.is_null());
        spring_config_close(config);
    }
}

#[test]
fn test_spring_config_toml() {
    let toml = CString::new("").unwrap();
    unsafe {
        let config = spring_config_toml(toml.as_ptr());
        assert!(!config.is_null());
        spring_config_close(config);
    }
}
