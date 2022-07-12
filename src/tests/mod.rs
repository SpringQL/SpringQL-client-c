// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use std::ffi::CString;

use crate::*;

#[test]
fn test_spring_config() {
    unsafe {
        let config = spring_config_default();
        assert!(!config.is_null());
        spring_config_close(config);
    }
}

#[test]
fn test_spring_source_row_builder() {
    unsafe {
        let builder = spring_source_row_builder();

        let c1_col = CString::new("c1").unwrap();
        let c1_value = vec![0x01, 0x02, 0x03];
        let errno = spring_source_row_add_column_blob(
            builder,
            c1_col.as_ptr(),
            c1_value.as_ptr().cast(),
            c1_value.len().try_into().unwrap(),
        );
        assert_eq!(errno, SpringErrno::Ok);

        let row = spring_source_row_build(builder);
        spring_source_row_close(row);
    }
}
