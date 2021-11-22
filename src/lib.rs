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
///
/// # Returns
///
/// - `0`: if there are no recent errors.
/// - `< 0`: SpringErrno
#[no_mangle]
pub extern "C" fn spring_open(mut pipeline: *mut SpringPipeline) -> SpringErrno {
    springql_core::spring_open()
        .map(|p| {
            pipeline = Box::into_raw(Box::new(SpringPipeline(p)));
            SpringErrno::Ok
        })
        .unwrap_or_else(update_last_error)
}

/// # Returns
///
/// - `0`: if there are no recent errors.
/// - `< 0`: SpringErrno
///
/// # Safety
///
/// This function is unsafe because it cast `*mut pipeline` into `&mut`.
#[no_mangle]
pub unsafe extern "C" fn spring_close(pipeline: *mut SpringPipeline) -> SpringErrno {
    if pipeline.is_null() {
        SpringErrno::CNull
    } else {
        drop(Box::from_raw(pipeline));
        SpringErrno::Ok
    }
}
