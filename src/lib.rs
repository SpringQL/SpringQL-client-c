//! C-API

use spring_last_errmsg::update_last_error;
use springql_core::low_level_rs as springql_core;

use spring_errno::SpringErrno;

pub mod spring_errno;
pub mod spring_last_errmsg;

#[non_exhaustive]
#[repr(transparent)]
pub struct SpringPipeline(springql_core::SpringPipeline);

/// See: springql_core::api::spring_open
#[no_mangle]
pub extern "C" fn spring_open(pipeline: *mut SpringPipeline) -> SpringErrno {
    springql_core::spring_open()
        .map(|p| unsafe {
            *pipeline = SpringPipeline(p);
            SpringErrno::Ok
        })
        .unwrap_or_else(update_last_error)
}
