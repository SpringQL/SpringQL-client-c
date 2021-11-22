use std::{
    cell::RefCell,
    error::Error,
    os::raw::{c_char, c_int},
    ptr, slice,
};

use log::warn;

use crate::spring_errno::SpringErrno;

thread_local! {
    static LAST_ERROR: RefCell<Option<Box<dyn Error>>> = RefCell::new(None);
}

fn take_last_error() -> Option<Box<dyn Error>> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}

/// Write the most recent error message into a caller-provided buffer as a UTF-8
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
/// 
/// # Safety
/// 
/// This function is unsafe because it writes into a caller-provided buffer.
#[no_mangle]
pub unsafe extern "C" fn spring_last_errmsg(buffer: *mut c_char, length: c_int) -> c_int {
    if buffer.is_null() {
        warn!("Null pointer passed into spring_last_errmsg() as the buffer");
        return SpringErrno::CNull as c_int;
    }

    let last_error = match take_last_error() {
        Some(err) => err,
        None => return SpringErrno::Ok as c_int,
    };

    let error_message = last_error.to_string();

    let buffer = slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if error_message.len() >= buffer.len() {
        warn!("Buffer provided for writing the last error message is too small.");
        warn!(
            "Expected at least {} bytes but got {}",
            error_message.len() + 1,
            buffer.len()
        );
        return SpringErrno::CInsufficient as c_int;
    }

    ptr::copy_nonoverlapping(
        error_message.as_ptr(),
        buffer.as_mut_ptr(),
        error_message.len(),
    );

    // Add a trailing null so people using the string as a `char *` don't
    // accidentally read into garbage.
    buffer[error_message.len()] = 0;

    error_message.len() as c_int
}

/// Calculate the number of bytes in the last error's error message **not**
/// including any trailing `null` characters.
/// 
/// # Returns
///
/// - `0`: if there are no recent errors.
/// - `> 0`: the length of the recent error message.
#[no_mangle]
pub extern "C" fn spring_last_errlen() -> c_int {
    LAST_ERROR.with(|prev| match *prev.borrow() {
        Some(ref err) => err.to_string().len() as c_int + 1,
        None => 0,
    })
}
