// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use crate::*;

#[test]
fn test_spring_source_row_builder() {
    unsafe {
        let builder = spring_source_row_builder();

        let c1 = vec![0x01, 0x02, 0x03];
        let errno = spring_source_row_add_column_blob(
            builder,
            "c1".as_ptr().cast(),
            c1.as_ptr().cast(),
            c1.len().try_into().unwrap(),
        );
        assert_eq!(errno as i32, SpringErrno::Ok as i32);

        let row = spring_source_row_build(builder);
        spring_source_row_close(row);
    }
}
