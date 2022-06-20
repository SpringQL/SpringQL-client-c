// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use std::{
    any::Any,
    cell::RefCell,
    error::Error,
    fmt::Display,
    os::raw::{c_char, c_int},
};

use ::log::{info, warn};
use ::springql_core::api::error::SpringError;

use crate::{c_mem::strcpy, spring_errno::SpringErrno};

thread_local! {
    static LAST_ERROR: RefCell<Option<LastError>> = RefCell::new(None);
}

fn take_last_error() -> Option<LastError> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}

#[derive(Debug)]
pub(super) enum LastError {
    SpringErr(SpringError),
    UnwindErr(Box<dyn Any + Send + 'static>),
}

impl Error for LastError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LastError::SpringErr(e) => e.source(),
            LastError::UnwindErr(_) => None,
        }
    }
}
impl Display for LastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LastError::SpringErr(e) => format!("{:?}", e),
            LastError::UnwindErr(any) => {
                if let Some(s) = any.downcast_ref::<String>() {
                    s.clone()
                } else if let Some(s) = any.downcast_ref::<&str>() {
                    s.to_string()
                } else {
                    "a panic occurred".to_string()
                }
            }
        };
        write!(f, "{}", s)
    }
}

/// Update the most recent error, clearing whatever may have been there before.
pub(super) fn update_last_error(err: LastError) {
    info!("Setting LAST_ERROR: {}", err);

    {
        // Print a pseudo-backtrace for this error, following back each error's
        // cause until we reach the root error.
        let mut source = err.source();
        while let Some(parent_err) = source {
            warn!("Caused by: {}", parent_err);
            source = parent_err.source();
        }
    }

    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(err);
    });
}

/// Write the most recent error number into `errno` and message into a caller-provided buffer as a UTF-8
/// string, returning the number of bytes written.
///
/// # Note
///
/// This writes a **UTF-8** string into the buffer. Windows users may need to
/// convert it to a UTF-16 "unicode" afterwards.
///
/// If there are no recent errors then this returns `0` (because we wrote 0
/// bytes). `-1` is returned if there are any errors, for example when passed a
/// null pointer or a buffer of insufficient size.
///
/// # Returns
///
/// - `0`: if there are no recent errors.
/// - `> 0`: the length of the recent error message.
/// - `< 0`: SpringErrno
#[no_mangle]
pub unsafe extern "C" fn spring_last_err(
    errno: *mut SpringErrno,
    errmsg: *mut c_char,
    errmsg_len: c_int,
) -> c_int {
    if errmsg.is_null() {
        warn!("Null pointer passed into spring_last_err() as the buffer");
        return SpringErrno::CNull as c_int;
    }

    let last_error = match take_last_error() {
        Some(err) => err,
        None => {
            *errno = SpringErrno::Ok;
            return SpringErrno::Ok as c_int;
        }
    };

    *errno = SpringErrno::from(&last_error);
    let error_message = last_error.to_string();

    strcpy(&error_message, errmsg, errmsg_len)
}

/// Calculate the number of bytes in the last error's error message **not**
/// including any trailing `null` characters.
///
/// # Returns
///
/// - `0`: if there are no recent errors.
/// - `> 0`: the length of the recent error message.
#[no_mangle]
pub extern "C" fn spring_last_errmsg_len() -> c_int {
    LAST_ERROR.with(|prev| match *prev.borrow() {
        Some(ref err) => err.to_string().len() as c_int + 1,
        None => 0,
    })
}
