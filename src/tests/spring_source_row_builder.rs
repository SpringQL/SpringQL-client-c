// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use std::ffi::CString;

use crate::*;

#[test]
fn test_spring_source_row_builder() {
    unsafe {
        let c1_col = CString::new("c1").unwrap();
        let c1_value = vec![0x01u8, 0x02, 0x03];

        let builder = spring_source_row_builder();
        let builder = spring_source_row_add_column_blob(
            builder,
            c1_col.as_ptr(),
            c1_value.as_ptr().cast(),
            c1_value.len().try_into().unwrap(),
        );
        assert!(!builder.is_null());

        let _row = spring_source_row_build(builder);
    }
}
