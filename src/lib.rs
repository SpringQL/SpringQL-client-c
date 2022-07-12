// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

//! C-API

#![allow(clippy::missing_safety_doc)] // C header file does not need `Safety` section

pub(crate) mod c_mem;

pub mod spring_config;
pub mod spring_errno;
pub mod spring_last_err;
mod spring_pipeline;
mod spring_sink_row;
mod spring_source_row;
mod spring_source_row_builder;

#[cfg(test)]
mod tests;

use std::{
    ffi::{c_void, CStr},
    os::raw::{c_char, c_float, c_int, c_long, c_short, c_uint},
    panic::{catch_unwind, UnwindSafe},
    ptr, slice,
};

use crate::{
    c_mem::{memcpy, strcpy},
    spring_config::SpringConfig,
    spring_errno::SpringErrno,
    spring_last_err::{update_last_error, LastError},
    spring_pipeline::SpringPipeline,
    spring_sink_row::SpringSinkRow,
    spring_source_row::SpringSourceRow,
    spring_source_row_builder::SpringSourceRowBuilder,
};
use ::springql::{
    error::SpringError, SpringPipeline as Pipeline, SpringSourceRow as RuSpringSourceRow,
    SpringSourceRowBuilder as RuSpringSourceRowBuilder,
};

/// Returns default configuration.
///
/// Returned value is not modifiable (it is just a void pointer).
/// If you would like to change the default configuration, use `spring_config_toml()` instead.
#[no_mangle]
pub extern "C" fn spring_config_default() -> *mut SpringConfig {
    let config = SpringConfig::default();
    config.into_ptr()
}

/// Configuration by TOML format string.
///
/// Returned value is not modifiable (it is just a void pointer).
///
/// # Parameters
///
/// - `overwrite_config_toml`: TOML format configuration to overwrite default.
///   See <https://springql.github.io/deployment/configuration> for TOML format and configuration values.
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

    let config = SpringConfig::from_toml(s).expect("failed to parse TOML config");
    config.into_ptr()
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
        let _ = Box::from_raw(config);
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
    let config = &*config;
    let res_ru_pipeline = with_catch(|| Pipeline::new(config.as_ref()));
    match res_ru_pipeline {
        Ok(ru_pipeline) => SpringPipeline::from(ru_pipeline).into_ptr(),
        Err(_err) => ptr::null_mut(),
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
        let _ = Box::from_raw(pipeline);
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
    let ru_pipeline = (*pipeline).as_ref();
    let sql = CStr::from_ptr(sql).to_string_lossy().into_owned();
    let result = with_catch(|| ru_pipeline.command(sql));

    match result {
        Ok(_) => SpringErrno::Ok,
        Err(e) => e,
    }
}

/// Pop a row from an in memory queue. This is a blocking function.
///
/// Do not call this function from threads.
/// If you need to pop from multiple in-memory queues using threads, use `spring_pop_non_blocking()`.
/// See: <https://github.com/SpringQL/SpringQL/issues/125>
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
) -> *mut SpringSinkRow {
    let ru_pipeline = (*pipeline).as_ref();
    let queue = CStr::from_ptr(queue).to_string_lossy().into_owned();
    let result = with_catch(|| ru_pipeline.pop(&queue));
    match result {
        Ok(ru_row) => {
            let row = SpringSinkRow::from(ru_row);
            row.into_ptr()
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
) -> *mut SpringSinkRow {
    let ru_pipeline = (*pipeline).as_ref();
    let queue = CStr::from_ptr(queue).to_string_lossy().into_owned();
    let result = with_catch(|| ru_pipeline.pop_non_blocking(&queue));
    match result {
        Ok(Some(row)) => {
            *is_err = false;
            SpringSinkRow::from(row).into_ptr()
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

/// Push a row into an in memory queue. This is a non-blocking function.
///
/// `row` is freed internally.
///
/// # Returns
///
/// - `Ok`: on success.
/// - `Unavailable`: queue named `queue` does not exist.
#[no_mangle]
pub unsafe extern "C" fn spring_push(
    pipeline: *const SpringPipeline,
    queue: *const c_char,
    row: *mut SpringSourceRow,
) -> SpringErrno {
    let ru_pipeline = (*pipeline).as_ref();
    let queue = CStr::from_ptr(queue).to_string_lossy().into_owned();

    let source_row = Box::from_raw(row);
    let source_row = RuSpringSourceRow::from(*source_row);
    let result = with_catch(|| ru_pipeline.push(&queue, source_row));
    match result {
        Ok(()) => SpringErrno::Ok,
        Err(e) => e,
    }
}

/// Create a source row from JSON string
///
/// # Returns
///
/// - non-NULL: Successfully created a row.
/// - NULL: Error occurred.
///
/// # Errors
///
/// - `InvalidFormat`: JSON string is invalid.
#[no_mangle]
pub unsafe extern "C" fn spring_source_row_from_json(json: *const c_char) -> *mut SpringSourceRow {
    let json = CStr::from_ptr(json).to_string_lossy().into_owned();
    let res_ru_source_row = with_catch(|| ::springql::SpringSourceRow::from_json(&json));
    match res_ru_source_row {
        Ok(ru_source_row) => SpringSourceRow::from(ru_source_row).into_ptr(),
        Err(_) => ptr::null_mut(),
    }
}

/// Start creating a source row using a builder.
///
/// # Returns
///
/// Pointer to the builder
#[no_mangle]
pub unsafe extern "C" fn spring_source_row_builder() -> *mut SpringSourceRowBuilder {
    SpringSourceRowBuilder::default().into_ptr()
}
/// Add a BLOB column to the builder and return the new one.
///
/// `builder` is freed internally.
///
/// # Parameters
///
/// - `builder`: Pointer to the builder created via spring_source_row_builder().
/// - `column_name`: Column name to add.
/// - `v`: BLOB value to add. The byte sequence is copied internally.
/// - `v_len`: `v`'s length.
///
/// # Returns
///
/// - non-NULL: Successfully created a row.
/// - NULL: Error occurred.
///
/// # Errors
///
/// - `Sql`: `column_name` is already added to the builder.
#[no_mangle]
pub unsafe extern "C" fn spring_source_row_add_column_blob(
    builder: *mut SpringSourceRowBuilder,
    column_name: *const c_char,
    v: *const c_void,
    v_len: c_int,
) -> *mut SpringSourceRowBuilder {
    let column_name = CStr::from_ptr(column_name).to_string_lossy().into_owned();

    let v = v as *const u8;
    let v = slice::from_raw_parts(v, v_len as usize);
    let v = v.to_vec();

    let builder = Box::from_raw(builder);
    let ru_builder = RuSpringSourceRowBuilder::from(*builder);
    let res_ru_builder = with_catch(|| ru_builder.add_column(column_name, v));
    match res_ru_builder {
        Ok(ru_builder) => SpringSourceRowBuilder::from(ru_builder).into_ptr(),
        Err(_) => ptr::null_mut(),
    }
}
/// Finish creating a source row using a builder.
///
/// The heap space for the `builder` is internally freed.
///
/// # Returns
///
/// SpringSourceRow
#[no_mangle]
pub unsafe extern "C" fn spring_source_row_build(
    builder: *mut SpringSourceRowBuilder,
) -> *mut SpringSourceRow {
    let builder = Box::from_raw(builder);
    let ru_builder = RuSpringSourceRowBuilder::from(*builder);
    SpringSourceRow::from(ru_builder.build()).into_ptr()
}

/// Frees heap occupied by a `SpringSinkRow`.
///
/// # Returns
///
/// - `Ok`: on success.
/// - `CNull`: `pipeline` is a NULL pointer.
#[no_mangle]
pub unsafe extern "C" fn spring_sink_row_close(row: *mut SpringSinkRow) -> SpringErrno {
    if row.is_null() {
        SpringErrno::CNull
    } else {
        let _ = Box::from_raw(row);
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
    row: *const SpringSinkRow,
    i_col: u16,
    out: *mut c_short,
) -> SpringErrno {
    let row = &*row;
    let i_col = i_col as usize;
    let result = with_catch(|| row.get_not_null_by_index(i_col as usize));
    match result {
        Ok(v) => {
            *out = v;
            SpringErrno::Ok
        }
        Err(e) => e,
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
    row: *const SpringSinkRow,
    i_col: u16,
    out: *mut c_int,
) -> SpringErrno {
    let row = &*row;
    let i_col = i_col as usize;
    let result = with_catch(|| row.get_not_null_by_index(i_col as usize));
    match result {
        Ok(v) => {
            *out = v;
            SpringErrno::Ok
        }
        Err(e) => e,
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
    row: *const SpringSinkRow,
    i_col: u16,
    out: *mut c_long,
) -> SpringErrno {
    let row = &*row;
    let i_col = i_col as usize;
    let result = with_catch(|| row.get_not_null_by_index(i_col as usize));
    match result {
        Ok(v) => {
            *out = v;
            SpringErrno::Ok
        }
        Err(e) => e,
    }
}

/// Get a 4-byte unsigned integer column.
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
pub unsafe extern "C" fn spring_column_unsigned_int(
    row: *const SpringSinkRow,
    i_col: u16,
    out: *mut c_uint,
) -> SpringErrno {
    let row = &*row;
    let i_col = i_col as usize;
    let result = with_catch(|| row.get_not_null_by_index(i_col as usize));
    match result {
        Ok(v) => {
            *out = v;
            SpringErrno::Ok
        }
        Err(e) => e,
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
    row: *const SpringSinkRow,
    i_col: u16,
    out: *mut c_char,
    out_len: c_int,
) -> c_int {
    let row = &*row;
    let i_col = i_col as usize;
    let result: Result<String, SpringErrno> =
        with_catch(|| row.get_not_null_by_index(i_col as usize));
    match result {
        Ok(v) => {
            let v_len = strcpy(&v, out, out_len);
            v_len as c_int
        }
        Err(e) => e as c_int,
    }
}

/// Get a BLOB column.
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
pub unsafe extern "C" fn spring_column_blob(
    row: *const SpringSinkRow,
    i_col: u16,
    out: *mut c_void,
    out_len: c_int,
) -> c_int {
    let row = &*row;
    let i_col = i_col as usize;
    let result: Result<Vec<u8>, SpringErrno> =
        with_catch(|| row.get_not_null_by_index(i_col as usize));
    match result {
        Ok(v) => {
            let v_len = memcpy(&v, out, out_len);
            v_len as c_int
        }
        Err(e) => e as c_int,
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
    row: *const SpringSinkRow,
    i_col: u16,
    out: *mut bool,
) -> SpringErrno {
    let row = &*row;
    let i_col = i_col as usize;
    let result = with_catch(|| row.get_not_null_by_index(i_col as usize));
    match result {
        Ok(v) => {
            *out = v;
            SpringErrno::Ok
        }
        Err(e) => e,
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
    row: *const SpringSinkRow,
    i_col: u16,
    out: *mut c_float,
) -> SpringErrno {
    let row = &*row;
    let i_col = i_col as usize;
    let result = with_catch(|| row.get_not_null_by_index(i_col as usize));
    match result {
        Ok(v) => {
            *out = v;
            SpringErrno::Ok
        }
        Err(e) => e,
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
