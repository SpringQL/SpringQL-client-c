// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

//! C-API

#![allow(clippy::missing_safety_doc)] // C header file does not need `Safety` section

use std::{
    convert::identity,
    ffi::CStr,
    mem,
    os::raw::{c_char, c_float, c_int, c_long, c_short},
    panic::{catch_unwind, UnwindSafe},
    ptr,
};

use ::springql_core::error::SpringError;
use cstr::strcpy;
use spring_last_err::{update_last_error, LastError};
use springql_core::{
    high_level_rs::{self as hl_api, SpringPipelineHL, SpringRowHL},
    low_level_rs as ll_api,
};

use spring_errno::SpringErrno;

pub mod spring_errno;
pub mod spring_last_err;

pub(crate) mod cstr;

mod conf {
    use springql_core::low_level_rs as ll_api;
    use std::{ffi::c_void, mem};
    /// Configuration.
    #[non_exhaustive]
    #[repr(transparent)]
    pub struct SpringConfig(*mut c_void);

    impl SpringConfig {
        pub fn new(config: ll_api::SpringConfig) -> Self {
            SpringConfig(unsafe { mem::transmute(Box::new(config)) })
        }

        pub fn llconf(&self) -> &ll_api::SpringConfig {
            unsafe { &*(self.0 as *const ll_api::SpringConfig) }
        }

        pub fn drop(ptr: *mut SpringConfig) {
            let outer = unsafe { Box::from_raw(ptr) };
            let inner = unsafe { Box::from_raw(outer.0) };
            drop(inner);
            drop(outer);
        }

        pub fn into_ptr(self) -> *mut SpringConfig {
            Box::into_raw(Box::new(self))
        }
    }
}
use conf::SpringConfig;
mod pipe {
    use springql_core::low_level_rs as ll_api;
    use std::{ffi::c_void, mem};

    /// Pipeline (dataflow definition) in SpringQL.
    #[non_exhaustive]
    #[repr(transparent)]
    pub struct SpringPipeline(*mut c_void);

    impl SpringPipeline {
        pub fn new(pipeline: ll_api::SpringPipeline) -> Self {
            SpringPipeline(unsafe { mem::transmute(Box::new(pipeline)) })
        }

        pub fn llpipe(&self) -> &ll_api::SpringPipeline {
            unsafe { &*(self.0 as *const ll_api::SpringPipeline) }
        }

        pub fn drop(ptr: *mut SpringPipeline) {
            let outer = unsafe { Box::from_raw(ptr) };
            let inner = unsafe { Box::from_raw(outer.0) };
            drop(inner);
            drop(outer);
        }

        pub fn into_ptr(self) -> *mut SpringPipeline {
            Box::into_raw(Box::new(self))
        }
    }
}
use pipe::SpringPipeline;

mod row {
    use springql_core::low_level_rs as ll_api;
    use std::{ffi::c_void, mem};

    /// Row object from an in memory queue.
    #[non_exhaustive]
    #[repr(transparent)]
    pub struct SpringRow(*mut c_void);

    impl SpringRow {
        pub fn new(row: ll_api::SpringRow) -> Self {
            SpringRow(unsafe { mem::transmute(Box::new(row)) })
        }

        pub fn llrow(&self) -> &ll_api::SpringRow {
            unsafe { &*(self.0 as *const ll_api::SpringRow) }
        }
        pub fn drop(ptr: *mut SpringRow) {
            let outer = unsafe { Box::from_raw(ptr) };
            let inner = unsafe { Box::from_raw(outer.0) };
            drop(inner);
            drop(outer);
        }
        pub fn into_ptr(self) -> *mut SpringRow {
            Box::into_raw(Box::new(self))
        }
    }
}
use row::SpringRow;

/// Returns default configuration.
///
/// Returned value is not modifiable (it is just a void pointer).
/// If you would like to change the default configuration, use `spring_config_toml()` instead.
#[no_mangle]
pub extern "C" fn spring_config_default() -> *mut SpringConfig {
    let config = ll_api::spring_config_default();
    SpringConfig::new(config).into_ptr()
}

/// Configuration by TOML format string.
///
/// Returned value is not modifiable (it is just a void pointer).
///
/// # Parameters
///
/// - `overwrite_config_toml`: TOML format configuration to overwrite default.
///   See https://springql.github.io/deployment/configuration for TOML format and configuration values.
///
/// # Panics
///
/// Currently, the process aborts when:
///
/// - `overwrite_config_toml` includes invalid key and/or value.
/// - `overwrite_config_toml` is not valid as TOML.
#[no_mangle]
pub unsafe extern "C" fn spring_config_toml(
    overwrite_config_toml: *const c_char,
) -> *mut SpringConfig {
    let s = CStr::from_ptr(overwrite_config_toml);
    let s = s.to_str().expect("failed to parse TOML string into UTF-8");

    let config = ll_api::spring_config_toml(s).expect("failed to parse TOML config");
    SpringConfig::new(config).into_ptr()
}

/// Frees heap occupied by a `SpringConfig`.
///
/// # Returns
///
/// - `Ok`: on success.
/// - `CNull`: `config` is a NULL pointer.
#[no_mangle]
pub unsafe extern "C" fn spring_config_close(config: *mut SpringConfig) -> SpringErrno {
    if config.is_null() {
        SpringErrno::CNull
    } else {
        SpringConfig::drop(config);
        SpringErrno::Ok
    }
}

/// Creates and open an in-process stream pipeline.
///
/// # Returns
///
/// - non-NULL: on success
/// - NULL: on failure. Check spring_last_err() for details.
///
/// # Errors
///
/// No errors are expected currently.
#[no_mangle]
pub unsafe extern "C" fn spring_open(config: *const SpringConfig) -> *mut SpringPipeline {
    let config = (*config).llconf();

    with_catch(|| ll_api::spring_open(config)).map_or_else(
        |_| ptr::null_mut(),
        |pipeline| SpringPipeline::new(pipeline).into_ptr(),
    )
}

/// Creates and open an in-process stream pipeline.
///
/// # Returns
///
/// - non-NULL: on success
/// - NULL: on failure. Check spring_last_err() for details.
///
/// # Errors
///
/// No errors are expected currently.
#[no_mangle]
pub unsafe extern "C" fn spring_open_hl(config: *const SpringConfig) -> *mut SpringPipelineHL {
    let config = (*config).llconf();
    let pipeline = hl_api::SpringPipelineHL::new(config);
    match pipeline {
        Ok(pipe) => {
            let mut boxed = Box::new(pipe);
            let ptr = (&mut (*boxed)) as *mut SpringPipelineHL;
            mem::forget(boxed);
            ptr
        }
        Err(_err) => return ptr::null_mut(),
    }
}

/// Frees heap occupied by a `SpringPipeline`.
///
/// # Returns
///
/// - `Ok`: on success.
/// - `CNull`: `pipeline` is a NULL pointer.
#[no_mangle]
pub unsafe extern "C" fn spring_close(pipeline: *mut SpringPipeline) -> SpringErrno {
    if pipeline.is_null() {
        SpringErrno::CNull
    } else {
        SpringPipeline::drop(pipeline);
        SpringErrno::Ok
    }
}

/// Frees heap occupied by a `SpringPipeline`.
///
/// # Returns
///
/// - `Ok`: on success.
/// - `CNull`: `pipeline` is a NULL pointer.
#[no_mangle]
pub unsafe extern "C" fn spring_close_hl(pipeline: *mut SpringPipelineHL) -> SpringErrno {
    if pipeline.is_null() {
        SpringErrno::CNull
    } else {
        let boxed = Box::from_raw(pipeline);
        drop(boxed);
        SpringErrno::Ok
    }
}

/// Execute commands (DDL) to modify the pipeline.
///
/// # Returns
///
/// - `Ok`: on success.
/// - `Sql`:
///   - Invalid SQL syntax.
///   - Refers to undefined objects (streams, pumps, etc)
///   - Other semantic errors.
/// - `InvalidOption`:
///   - `OPTIONS` in `CREATE` statement includes invalid key or value.
#[no_mangle]
pub unsafe extern "C" fn spring_command(
    pipeline: *const SpringPipeline,
    sql: *const c_char,
) -> SpringErrno {
    let pipeline = (*pipeline).llpipe();
    let sql = CStr::from_ptr(sql).to_string_lossy().into_owned();

    with_catch(|| ll_api::spring_command(pipeline, &sql))
        .map_or_else(identity, |()| SpringErrno::Ok)
}

/// Execute commands (DDL) to modify the pipeline.
///
/// # Returns
///
/// - `Ok`: on success.
/// - `Sql`:
///   - Invalid SQL syntax.
///   - Refers to undefined objects (streams, pumps, etc)
///   - Other semantic errors.
/// - `InvalidOption`:
///   - `OPTIONS` in `CREATE` statement includes invalid key or value.
#[no_mangle]
pub unsafe extern "C" fn spring_command_hl(
    pipeline: *const SpringPipelineHL,
    sql: *const c_char,
) -> SpringErrno {
    let boxed = Box::from_raw(pipeline as *mut SpringPipelineHL);
    let sql = CStr::from_ptr(sql).to_string_lossy().into_owned();
    let result = boxed.command(sql);
    mem::forget(boxed); // keep SpringPipelineHL instance

    match result {
        Ok(_) => SpringErrno::Ok,
        Err(SpringError::Sql(_)) => SpringErrno::Sql,
        Err(SpringError::InvalidOption {
            key: _,
            value: _,
            source: _,
        }) => SpringErrno::InvalidOption,
        Err(_) => SpringErrno::Unknown,
    }
}

/// Pop a row from an in memory queue. This is a blocking function.
///
/// Do not call this function from threads.
/// If you need to pop from multiple in-memory queues using threads, use `spring_pop_non_blocking()`.
/// See: https://github.com/SpringQL/SpringQL/issues/125
///
/// # Returns
///
/// - non-NULL: on success
/// - NULL: on failure. Check spring_last_err() for details.
///
/// # Errors
///
/// - `Unavailable`: queue named `queue` does not exist.
#[no_mangle]
pub unsafe extern "C" fn spring_pop(
    pipeline: *const SpringPipeline,
    queue: *const c_char,
) -> *mut SpringRow {
    let pipeline = (*pipeline).llpipe();
    let queue = CStr::from_ptr(queue).to_string_lossy().into_owned();

    with_catch(|| ll_api::spring_pop(pipeline, &queue))
        .map_or_else(|_| ptr::null_mut(), |row| SpringRow::new(row).into_ptr())
}

/// Pop a row from an in memory queue. This is a blocking function.
///
/// Do not call this function from threads.
/// If you need to pop from multiple in-memory queues using threads, use `spring_pop_non_blocking()`.
/// See: https://github.com/SpringQL/SpringQL/issues/125
///
/// # Returns
///
/// - non-NULL: on success
/// - NULL: on failure. Check spring_last_err() for details.
///
/// # Errors
///
/// - `Unavailable`: queue named `queue` does not exist.
#[no_mangle]
pub unsafe extern "C" fn spring_pop_hl(
    pipeline: *const SpringPipeline,
    queue: *const c_char,
) -> *mut SpringRowHL {
    let boxed = Box::from_raw(pipeline as *mut SpringPipelineHL);
    let queue = CStr::from_ptr(queue).to_string_lossy().into_owned();
    let result = boxed.pop(&queue);
    mem::forget(boxed); // keep SpringPipelineHL instance
    match result {
        Ok(row) => {
            let mut boxed_row = Box::new(row);
            let ptr = (&mut *boxed_row) as *mut SpringRowHL;
            mem::forget(boxed_row);
            ptr
        }
        Err(_) => ptr::null_mut(),
    }
}

/// Pop a row from an in memory queue. This is a non-blocking function.
///
/// # Returns
///
/// - non-NULL: Successfully get a row.
/// - NULL: Error occurred if `is_err` is true (check spring_last_err() for details). Otherwise, any row is not in the queue.
///
/// # Errors
///
/// - `Unavailable`: queue named `queue` does not exist.
#[no_mangle]
pub unsafe extern "C" fn spring_pop_non_blocking(
    pipeline: *const SpringPipeline,
    queue: *const c_char,
    is_err: *mut bool,
) -> *mut SpringRow {
    *is_err = false;

    let pipeline = (*pipeline).llpipe();
    let queue = CStr::from_ptr(queue).to_string_lossy().into_owned();

    with_catch(|| ll_api::spring_pop_non_blocking(pipeline, &queue)).map_or_else(
        |_| {
            *is_err = true;
            ptr::null_mut()
        },
        |opt_row| {
            if let Some(row) = opt_row {
                SpringRow::new(row).into_ptr()
            } else {
                ptr::null_mut()
            }
        },
    )
}

/// Pop a row from an in memory queue. This is a non-blocking function.
///
/// # Returns
///
/// - non-NULL: Successfully get a row.
/// - NULL: Error occurred if `is_err` is true (check spring_last_err() for details). Otherwise, any row is not in the queue.
///
/// # Errors
///
/// - `Unavailable`: queue named `queue` does not exist.
#[no_mangle]
pub unsafe extern "C" fn spring_pop_non_blocking_hl(
    pipeline: *const SpringPipelineHL,
    queue: *const c_char,
    is_err: *mut bool,
) -> *mut SpringRowHL {
    let boxed = Box::from_raw(pipeline as *mut SpringPipelineHL);
    let queue = CStr::from_ptr(queue).to_string_lossy().into_owned();
    let result = boxed.pop_non_blocking(&queue);
    mem::forget(boxed); // keep SpringPipelineHL instance
    match result {
        Ok(Some(row)) => {
            let mut boxed_row = Box::new(row);
            let ptr = (&mut *boxed_row) as *mut SpringRowHL;
            mem::forget(boxed_row);
            *is_err = false;
            ptr
        }
        Ok(None) => {
            *is_err = false;
            ptr::null_mut()
        }
        Err(_) => {
            *is_err = true;
            ptr::null_mut()
        }
    }
}

/// Frees heap occupied by a `SpringRow`.
///
/// # Returns
///
/// - `Ok`: on success.
/// - `CNull`: `pipeline` is a NULL pointer.
#[no_mangle]
pub extern "C" fn spring_row_close(row: *mut SpringRow) -> SpringErrno {
    if row.is_null() {
        SpringErrno::CNull
    } else {
        SpringRow::drop(row);
        SpringErrno::Ok
    }
}

/// Frees heap occupied by a `SpringRow`.
///
/// # Returns
///
/// - `Ok`: on success.
/// - `CNull`: `pipeline` is a NULL pointer.
#[no_mangle]
pub extern "C" fn spring_row_close_hl(row: *mut SpringRowHL) -> SpringErrno {
    if row.is_null() {
        SpringErrno::CNull
    } else {
        let boxed = unsafe { Box::from_raw(row as *mut SpringRowHL) };
        mem::drop(boxed);
        SpringErrno::Ok
    }
}

/// Get a 2-byte integer column.
///
/// # Parameters
///
/// - `row`: A `SpringRow` pointer to get a column value from.
/// - `i_col`: The column index to get a value from.
/// - `out`: A pointer to a buffer to store the column value.
///
/// # Returns
///
/// - `Ok`: On success.
/// - `Unavailable`:
///   - Column pointed by `i_col` is already fetched.
///   - `i_col` is out of range.
/// - `CNull`: Column value is NULL.
#[no_mangle]
pub unsafe extern "C" fn spring_column_short(
    row: *const SpringRow,
    i_col: u16,
    out: *mut c_short,
) -> SpringErrno {
    let row = (*row).llrow();
    let i_col = i_col as usize;

    with_catch(|| ll_api::spring_column_i16(row, i_col)).map_or_else(identity, |r| {
        *out = r;
        SpringErrno::Ok
    })
}

/// Get a 2-byte integer column.
///
/// # Parameters
///
/// - `row`: A `SpringRow` pointer to get a column value from.
/// - `i_col`: The column index to get a value from.
/// - `out`: A pointer to a buffer to store the column value.
///
/// # Returns
///
/// - `Ok`: On success.
/// - `Unavailable`:
///   - Column pointed by `i_col` is already fetched.
///   - `i_col` is out of range.
/// - `CNull`: Column value is NULL.
#[no_mangle]
pub unsafe extern "C" fn spring_column_short_hl(
    row: *const SpringRowHL,
    i_col: u16,
    out: *mut c_short,
) -> SpringErrno {
    let boxed = Box::from_raw(row as *mut SpringRowHL);
    let i_col = i_col as usize;
    let result = boxed.get_not_null_by_index(i_col as usize);
    mem::forget(boxed);
    match result {
        Ok(v) => {
            *out = v;
            return SpringErrno::Ok;
        }
        Err(e) => return SpringErrno::from(&e),
    }
}

/// Get a 4-byte integer column.
///
/// # Parameters
///
/// - `row`: A `SpringRow` pointer to get a column value from.
/// - `i_col`: The column index to get a value from.
/// - `out`: A pointer to a buffer to store the column value.
///
/// # Returns
///
/// - `Ok`: On success.
/// - `Unavailable`:
///   - Column pointed by `i_col` is already fetched.
///   - `i_col` is out of range.
/// - `CNull`: Column value is NULL.
#[no_mangle]
pub unsafe extern "C" fn spring_column_int(
    row: *const SpringRow,
    i_col: u16,
    out: *mut c_int,
) -> SpringErrno {
    let row = (*row).llrow();
    let i_col = i_col as usize;

    with_catch(|| ll_api::spring_column_i32(row, i_col)).map_or_else(identity, |r| {
        *out = r;
        SpringErrno::Ok
    })
}

/// Get a 4-byte integer column.
///
/// # Parameters
///
/// - `row`: A `SpringRow` pointer to get a column value from.
/// - `i_col`: The column index to get a value from.
/// - `out`: A pointer to a buffer to store the column value.
///
/// # Returns
///
/// - `Ok`: On success.
/// - `Unavailable`:
///   - Column pointed by `i_col` is already fetched.
///   - `i_col` is out of range.
/// - `CNull`: Column value is NULL.
#[no_mangle]
pub unsafe extern "C" fn spring_column_int_hl(
    row: *const SpringRowHL,
    i_col: u16,
    out: *mut c_int,
) -> SpringErrno {
    let boxed = Box::from_raw(row as *mut SpringRowHL);
    let i_col = i_col as usize;
    let result = boxed.get_not_null_by_index(i_col as usize);
    mem::forget(boxed);
    match result {
        Ok(v) => {
            *out = v;
            return SpringErrno::Ok;
        }
        Err(e) => return SpringErrno::from(&e),
    }
}

/// Get an 8-byte integer column.
///
/// # Parameters
///
/// - `row`: A `SpringRow` pointer to get a column value from.
/// - `i_col`: The column index to get a value from.
/// - `out`: A pointer to a buffer to store the column value.
///
/// # Returns
///
/// - `Ok`: On success.
/// - `Unavailable`:
///   - Column pointed by `i_col` is already fetched.
///   - `i_col` is out of range.
/// - `CNull`: Column value is NULL.
#[no_mangle]
pub unsafe extern "C" fn spring_column_long(
    row: *const SpringRow,
    i_col: u16,
    out: *mut c_long,
) -> SpringErrno {
    let row = (*row).llrow();
    let i_col = i_col as usize;

    with_catch(|| ll_api::spring_column_i64(row, i_col)).map_or_else(identity, |r| {
        *out = r;
        SpringErrno::Ok
    })
}

/// Get an 8-byte integer column.
///
/// # Parameters
///
/// - `row`: A `SpringRow` pointer to get a column value from.
/// - `i_col`: The column index to get a value from.
/// - `out`: A pointer to a buffer to store the column value.
///
/// # Returns
///
/// - `Ok`: On success.
/// - `Unavailable`:
///   - Column pointed by `i_col` is already fetched.
///   - `i_col` is out of range.
/// - `CNull`: Column value is NULL.
#[no_mangle]
pub unsafe extern "C" fn spring_column_long_hl(
    row: *const SpringRowHL,
    i_col: u16,
    out: *mut c_long,
) -> SpringErrno {
    let boxed = Box::from_raw(row as *mut SpringRowHL);
    let i_col = i_col as usize;
    let result = boxed.get_not_null_by_index(i_col as usize);
    mem::forget(boxed);
    match result {
        Ok(v) => {
            *out = v;
            return SpringErrno::Ok;
        }
        Err(e) => return SpringErrno::from(&e),
    }
}

/// Get a text column.
///
/// # Parameters
///
/// - `row`: A `SpringRow` pointer to get a column value from.
/// - `i_col`: The column index to get a value from.
/// - `out`: A pointer to a buffer to store the column value.
/// - `out_len`: The length of the buffer pointed by `out`.
///
/// # Returns
///
/// - `> 0`: Length of the text.
/// - `Unavailable`:
///   - Column pointed by `i_col` is already fetched.
///   - `i_col` is out of range.
/// - `CNull`: Column value is NULL.
#[no_mangle]
pub unsafe extern "C" fn spring_column_text(
    row: *const SpringRow,
    i_col: u16,
    out: *mut c_char,
    out_len: c_int,
) -> c_int {
    let row = (*row).llrow();
    let i_col = i_col as usize;

    with_catch(|| ll_api::spring_column_text(row, i_col))
        .map_or_else(|errno| errno as c_int, |text| strcpy(&text, out, out_len))
}

/// Get a text column.
///
/// # Parameters
///
/// - `row`: A `SpringRow` pointer to get a column value from.
/// - `i_col`: The column index to get a value from.
/// - `out`: A pointer to a buffer to store the column value.
/// - `out_len`: The length of the buffer pointed by `out`.
///
/// # Returns
///
/// - `> 0`: Length of the text.
/// - `Unavailable`:
///   - Column pointed by `i_col` is already fetched.
///   - `i_col` is out of range.
/// - `CNull`: Column value is NULL.
#[no_mangle]
pub unsafe extern "C" fn spring_column_text_hl(
    row: *const SpringRowHL,
    i_col: u16,
    out: *mut c_char,
    out_len: c_int,
) -> c_int {
    let boxed = Box::from_raw(row as *mut SpringRowHL);
    let i_col = i_col as usize;
    let result: Result<String, SpringError> = boxed.get_not_null_by_index(i_col as usize);
    mem::forget(boxed);
    match result {
        Ok(v) => {
            strcpy(&v, out, out_len);
            return v.len() as c_int;
        }
        Err(e) => return SpringErrno::from(&e) as c_int,
    }
}

/// Get a bool column.
///
/// # Parameters
///
/// - `row`: A `SpringRow` pointer to get a column value from.
/// - `i_col`: The column index to get a value from.
/// - `out`: A pointer to a buffer to store the column value.
///
/// # Returns
///
/// - `Ok`: On success.
/// - `Unavailable`:
///   - Column pointed by `i_col` is already fetched.
///   - `i_col` is out of range.
/// - `CNull`: Column value is NULL.
#[no_mangle]
pub unsafe extern "C" fn spring_column_bool(
    row: *const SpringRow,
    i_col: u16,
    out: *mut bool,
) -> SpringErrno {
    let row = (*row).llrow();
    let i_col = i_col as usize;

    with_catch(|| ll_api::spring_column_bool(row, i_col)).map_or_else(identity, |r| {
        *out = r;
        SpringErrno::Ok
    })
}

/// Get a bool column.
///
/// # Parameters
///
/// - `row`: A `SpringRow` pointer to get a column value from.
/// - `i_col`: The column index to get a value from.
/// - `out`: A pointer to a buffer to store the column value.
///
/// # Returns
///
/// - `Ok`: On success.
/// - `Unavailable`:
///   - Column pointed by `i_col` is already fetched.
///   - `i_col` is out of range.
/// - `CNull`: Column value is NULL.
#[no_mangle]
pub unsafe extern "C" fn spring_column_bool_hl(
    row: *const SpringRowHL,
    i_col: u16,
    out: *mut bool,
) -> SpringErrno {
    let boxed = Box::from_raw(row as *mut SpringRowHL);
    let i_col = i_col as usize;
    let result = boxed.get_not_null_by_index(i_col as usize);
    mem::forget(boxed);
    match result {
        Ok(v) => {
            *out = v;
            return SpringErrno::Ok;
        }
        Err(e) => return SpringErrno::from(&e),
    }
}

/// Get a 4-byte floating point column.
///
/// # Parameters
///
/// - `row`: A `SpringRow` pointer to get a column value from.
/// - `i_col`: The column index to get a value from.
/// - `out`: A pointer to a buffer to store the column value.
///
/// # Returns
///
/// - `Ok`: On success.
/// - `Unavailable`:
///   - Column pointed by `i_col` is already fetched.
///   - `i_col` is out of range.
/// - `CNull`: Column value is NULL.
#[no_mangle]
pub unsafe extern "C" fn spring_column_float(
    row: *const SpringRow,
    i_col: u16,
    out: *mut c_float,
) -> SpringErrno {
    let row = (*row).llrow();
    let i_col = i_col as usize;

    with_catch(|| ll_api::spring_column_f32(row, i_col)).map_or_else(identity, |r| {
        *out = r;
        SpringErrno::Ok
    })
}

/// Get a 4-byte floating point column.
///
/// # Parameters
///
/// - `row`: A `SpringRow` pointer to get a column value from.
/// - `i_col`: The column index to get a value from.
/// - `out`: A pointer to a buffer to store the column value.
///
/// # Returns
///
/// - `Ok`: On success.
/// - `Unavailable`:
///   - Column pointed by `i_col` is already fetched.
///   - `i_col` is out of range.
/// - `CNull`: Column value is NULL.
#[no_mangle]
pub unsafe extern "C" fn spring_column_float_hl(
    row: *const SpringRowHL,
    i_col: u16,
    out: *mut c_float,
) -> SpringErrno {
    let boxed = Box::from_raw(row as *mut SpringRowHL);
    let i_col = i_col as usize;
    let result = boxed.get_not_null_by_index(i_col as usize);
    mem::forget(boxed);
    match result {
        Ok(v) => {
            *out = v;
            return SpringErrno::Ok;
        }
        Err(e) => return SpringErrno::from(&e),
    }
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
