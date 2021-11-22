//! C-API

use spring_errno::SpringErrno;

pub mod spring_errno;
pub mod spring_last_errmsg;

/// See: springql_core::api::spring_open
#[no_mangle]
pub extern "C" fn spring_open() -> SpringErrno {
    todo!()
}
