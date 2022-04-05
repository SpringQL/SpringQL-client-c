// This file is part of https://github.com/SpringQL/SpringQL which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use std::{
    os::raw::{c_char, c_int},
    ptr, slice,
};

use log::warn;

use crate::spring_errno::SpringErrno;

/// # Returns
///
/// - `> 0`: the length of the recent error message.
/// - `< 0`: SpringErrno
pub(super) fn strcpy(src: &str, dest_buf: *mut c_char, dest_len: c_int) -> c_int {
    if src.len() >= dest_len as usize {
        warn!("dest_len is smaller than src.");
        warn!(
            "Expected at least {} bytes but got {}",
            src.len() + 1,
            dest_len
        );
        return SpringErrno::CInsufficient as c_int;
    }

    let buffer = unsafe { slice::from_raw_parts_mut(dest_buf as *mut u8, dest_len as usize) };
    unsafe {
        ptr::copy_nonoverlapping(src.as_ptr(), buffer.as_mut_ptr(), src.len());
    }

    // Add a trailing null so people using the string as a `char *` don't
    // accidentally read into garbage.
    buffer[src.len()] = 0;

    src.len() as c_int
}
