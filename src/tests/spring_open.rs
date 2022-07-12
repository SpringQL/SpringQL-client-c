// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use crate::*;

#[test]
fn test_spring_open() {
    unsafe {
        let config = spring_config_default();
        assert!(!config.is_null());

        let pipeline = spring_open(config);
        assert!(!config.is_null());

        spring_close(pipeline);
        spring_config_close(config);
    }
}
