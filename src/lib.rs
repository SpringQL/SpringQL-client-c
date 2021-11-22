//! C-API

use std::{
    convert::identity,
    ffi::CStr,
    os::raw::{c_char, c_int},
    panic::{catch_unwind, UnwindSafe},
};

use ::springql_core::error::SpringError;
use cstr::strcpy;
use spring_last_errmsg::{update_last_error, LastError};
use springql_core::low_level_rs as springql_core;

use spring_errno::SpringErrno;

pub mod spring_errno;
pub mod spring_last_errmsg;

pub(crate) mod cstr;

#[non_exhaustive]
#[repr(transparent)]
pub struct SpringPipeline(springql_core::SpringPipeline);

#[non_exhaustive]
#[repr(transparent)]
pub struct SpringRow(springql_core::SpringRow);

/// See: springql_core::api::spring_open
///
/// # Returns
///
/// - `0`: if there are no recent errors.
/// - `< 0`: SpringErrno
#[no_mangle]
pub extern "C" fn spring_open(mut pipeline: *mut SpringPipeline) -> SpringErrno {
    with_catch(springql_core::spring_open).map_or_else(identity, |p| {
        pipeline = Box::into_raw(Box::new(SpringPipeline(p)));
        SpringErrno::Ok
    })
}

/// # Returns
///
/// - `0`: if there are no recent errors.
/// - `< 0`: SpringErrno
///
/// # Safety
///
/// This function is unsafe because it uses raw pointer.
#[no_mangle]
pub unsafe extern "C" fn spring_close(pipeline: *mut SpringPipeline) -> SpringErrno {
    if pipeline.is_null() {
        SpringErrno::CNull
    } else {
        drop(Box::from_raw(pipeline));
        SpringErrno::Ok
    }
}

/// See: springql_core::api::spring_command
///
/// # Returns
///
/// - `0`: if there are no recent errors.
/// - `< 0`: SpringErrno
///
/// # Safety
///
/// This function is unsafe because it uses raw pointer.
#[no_mangle]
pub unsafe extern "C" fn spring_command(
    pipeline: *const SpringPipeline,
    sql: *const c_char,
) -> SpringErrno {
    let pipeline = &*pipeline;
    let sql = CStr::from_ptr(sql).to_string_lossy().into_owned();

    with_catch(|| springql_core::spring_command(&pipeline.0, &sql))
        .map_or_else(identity, |()| SpringErrno::Ok)
}

/// See: springql_core::api::spring_pop
///
/// # Returns
///
/// - `0`: if there are no recent errors.
/// - `< 0`: SpringErrno
///
/// # Safety
///
/// This function is unsafe because it uses raw pointer.
#[no_mangle]
pub unsafe extern "C" fn spring_pop(
    pipeline: *const SpringPipeline,
    queue: *const c_char,
    mut row: *mut SpringRow,
) -> SpringErrno {
    let pipeline = &*pipeline;
    let queue = CStr::from_ptr(queue).to_string_lossy().into_owned();

    with_catch(|| springql_core::spring_pop(&pipeline.0, &queue)).map_or_else(identity, |r| {
        row = Box::into_raw(Box::new(SpringRow(r)));
        SpringErrno::Ok
    })
}

/// # Returns
///
/// - `0`: if there are no recent errors.
/// - `< 0`: SpringErrno
///
/// # Safety
///
/// This function is unsafe because it uses raw pointer.
#[no_mangle]
pub unsafe extern "C" fn spring_row_close(row: *mut SpringRow) -> SpringErrno {
    if row.is_null() {
        SpringErrno::CNull
    } else {
        drop(Box::from_raw(row));
        SpringErrno::Ok
    }
}

/// See: springql_core::api::spring_column_i32
///
/// # Returns
///
/// - `0`: if there are no recent errors.
/// - `< 0`: SpringErrno
///
/// # Safety
///
/// This function is unsafe because it uses raw pointer.
#[no_mangle]
pub unsafe extern "C" fn spring_column_int(
    row: *const SpringRow,
    i_col: u16,
    out: *mut c_int,
) -> SpringErrno {
    let row = &*row;
    let i_col = i_col as usize;

    with_catch(|| springql_core::spring_column_i32(&row.0, i_col)).map_or_else(identity, |r| {
        *out = r;
        SpringErrno::Ok
    })
}

/// See: springql_core::api::spring_column_text
///
/// This returns UTF-8 string into `out`.
///
/// # Returns
///
/// - `0`: if there are no recent errors.
/// - `> 0`: the length of the recent error message.
/// - `< 0`: SpringErrno
///
/// # Safety
///
/// This function is unsafe because it uses raw pointer.
#[no_mangle]
pub unsafe extern "C" fn spring_column_text(
    row: *const SpringRow,
    i_col: u16,
    out: *mut c_char,
    out_len: c_int,
) -> c_int {
    let row = &*row;
    let i_col = i_col as usize;

    with_catch(|| springql_core::spring_column_text(&row.0, i_col))
        .map_or_else(|errno| errno as c_int, |text| strcpy(&text, out, out_len))
}

fn with_catch<F, R>(f: F) -> Result<R, SpringErrno>
where
    F: FnOnce() -> Result<R, SpringError> + UnwindSafe,
{
    catch_unwind(|| f().map_err(LastError::SpringErr))
        .unwrap_or_else(|panic_err| Err(LastError::UnwindErr(panic_err)))
        .map_err(|last_err| {
            let errno = match &last_err {
                LastError::SpringErr(e) => SpringErrno::from(e),
                LastError::UnwindErr(_) => SpringErrno::Unknown,
            };
            update_last_error(last_err);
            errno
        })
}
