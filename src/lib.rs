// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

//! C-API

use std::{
    convert::identity,
    ffi::{c_void, CStr},
    mem,
    os::raw::{c_char, c_float, c_int, c_long, c_short},
    panic::{catch_unwind, UnwindSafe},
    ptr,
};

use ::springql_core::error::SpringError;
use cstr::strcpy;
use spring_last_err::{update_last_error, LastError};
use springql_core::low_level_rs as springql_core;

use spring_errno::SpringErrno;

pub mod spring_errno;
pub mod spring_last_err;

pub(crate) mod cstr;

#[non_exhaustive]
#[repr(transparent)]
pub struct SpringConfig(*mut c_void);

#[non_exhaustive]
#[repr(transparent)]
pub struct SpringPipeline(*mut c_void);

#[non_exhaustive]
#[repr(transparent)]
pub struct SpringRow(*mut c_void);

/// See: springql_core::api::spring_config_default
///
/// Returned value is not modifiable (it is just a void pointer).
/// If you would like to change the default configuration, use `spring_config_toml()` instead.
#[no_mangle]
pub extern "C" fn spring_config_default() -> *mut SpringConfig {
    let config = springql_core::spring_config_default();
    Box::into_raw(Box::new(SpringConfig(unsafe {
        mem::transmute(Box::new(config))
    })))
}

/// See: springql_core::api::spring_config_default
///
/// Returned value is not modifiable (it is just a void pointer).
/// If you would like to change the default configuration, use `spring_config_toml()` instead.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointer.
#[no_mangle]
pub unsafe extern "C" fn spring_config_toml(
    overwrite_config_toml: *const c_char,
) -> *mut SpringConfig {
    let s = { CStr::from_ptr(overwrite_config_toml) };
    let s = s.to_str().expect("failed to parse TOML string into UTF-8");

    let config = springql_core::spring_config_toml(s).expect("failed to parse TOML config");
    Box::into_raw(Box::new(SpringConfig(mem::transmute(Box::new(config)))))
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
pub unsafe extern "C" fn spring_config_close(config: *mut SpringConfig) -> SpringErrno {
    if config.is_null() {
        SpringErrno::CNull
    } else {
        let outer = Box::from_raw(config);
        let inner = Box::from_raw(outer.0);
        drop(inner);
        drop(outer);
        SpringErrno::Ok
    }
}

/// See: springql_core::api::spring_open
///
/// # Returns
///
/// - non-NULL: on success
/// - NULL: on failure. Check spring_last_err() for details.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointer.
#[no_mangle]
pub unsafe extern "C" fn spring_open(config: *const SpringConfig) -> *mut SpringPipeline {
    let config = &*((*config).0 as *const springql_core::SpringConfig);

    with_catch(|| springql_core::spring_open(config)).map_or_else(
        |_| ptr::null_mut(),
        |pipeline| Box::into_raw(Box::new(SpringPipeline(mem::transmute(Box::new(pipeline))))),
    )
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
        let outer = Box::from_raw(pipeline);
        let inner = Box::from_raw(outer.0);
        drop(inner);
        drop(outer);
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
    let pipeline = &*((*pipeline).0 as *const springql_core::SpringPipeline);
    let sql = CStr::from_ptr(sql).to_string_lossy().into_owned();

    with_catch(|| springql_core::spring_command(pipeline, &sql))
        .map_or_else(identity, |()| SpringErrno::Ok)
}

/// See: springql_core::api::spring_pop
///
/// # Returns
///
/// - non-NULL: on success
/// - NULL: on failure. Check spring_last_err() for details.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointer.
#[no_mangle]
pub unsafe extern "C" fn spring_pop(
    pipeline: *const SpringPipeline,
    queue: *const c_char,
) -> *mut SpringRow {
    let pipeline = &*((*pipeline).0 as *const springql_core::SpringPipeline);
    let queue = CStr::from_ptr(queue).to_string_lossy().into_owned();

    with_catch(|| springql_core::spring_pop(pipeline, &queue)).map_or_else(
        |_| ptr::null_mut(),
        |row| Box::into_raw(Box::new(SpringRow(mem::transmute(Box::new(row))))),
    )
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
        let outer = Box::from_raw(row);
        let inner = Box::from_raw(outer.0);
        drop(inner);
        drop(outer);
        SpringErrno::Ok
    }
}

/// See: springql_core::api::spring_column_i16
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
pub unsafe extern "C" fn spring_column_short(
    row: *const SpringRow,
    i_col: u16,
    out: *mut c_short,
) -> SpringErrno {
    let row = &*((*row).0 as *const springql_core::SpringRow);
    let i_col = i_col as usize;

    with_catch(|| springql_core::spring_column_i16(row, i_col)).map_or_else(identity, |r| {
        *out = r;
        SpringErrno::Ok
    })
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
    let row = &*((*row).0 as *const springql_core::SpringRow);
    let i_col = i_col as usize;

    with_catch(|| springql_core::spring_column_i32(row, i_col)).map_or_else(identity, |r| {
        *out = r;
        SpringErrno::Ok
    })
}

/// See: springql_core::api::spring_column_i64
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
pub unsafe extern "C" fn spring_column_long(
    row: *const SpringRow,
    i_col: u16,
    out: *mut c_long,
) -> SpringErrno {
    let row = &*((*row).0 as *const springql_core::SpringRow);
    let i_col = i_col as usize;

    with_catch(|| springql_core::spring_column_i64(row, i_col)).map_or_else(identity, |r| {
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
    let row = &*((*row).0 as *const springql_core::SpringRow);
    let i_col = i_col as usize;

    with_catch(|| springql_core::spring_column_text(row, i_col))
        .map_or_else(|errno| errno as c_int, |text| strcpy(&text, out, out_len))
}

/// See: springql_core::api::spring_column_bool
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
pub unsafe extern "C" fn spring_column_bool(
    row: *const SpringRow,
    i_col: u16,
    out: *mut bool,
) -> SpringErrno {
    let row = &*((*row).0 as *const springql_core::SpringRow);
    let i_col = i_col as usize;

    with_catch(|| springql_core::spring_column_bool(row, i_col)).map_or_else(identity, |r| {
        *out = r;
        SpringErrno::Ok
    })
}

/// See: springql_core::api::spring_column_f32
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
pub unsafe extern "C" fn spring_column_float(
    row: *const SpringRow,
    i_col: u16,
    out: *mut c_float,
) -> SpringErrno {
    let row = &*((*row).0 as *const springql_core::SpringRow);
    let i_col = i_col as usize;

    with_catch(|| springql_core::spring_column_f32(row, i_col)).map_or_else(identity, |r| {
        *out = r;
        SpringErrno::Ok
    })
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
