// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use crate::*;

#[test]
fn test_spring_config() {
    unsafe {
        let config = spring_config_default();
        assert!(!config.is_null());
        spring_config_close(config);
    }
}
