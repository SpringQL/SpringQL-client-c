// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use ::springql::SpringSourceRow as SourceRow;

use std::{ffi::c_void, mem};

/// Row object to push into an in memory queue.
#[non_exhaustive]
#[repr(transparent)]
pub struct SpringSourceRow(*mut c_void);

impl SpringSourceRow {
    pub fn new(source_row: SourceRow) -> Self {
        SpringSourceRow(unsafe { mem::transmute(Box::new(source_row)) })
    }

    pub fn to_row(&self) -> SourceRow {
        unsafe { &*(self.0 as *const SourceRow) }.clone()
    }

    pub fn drop(ptr: *mut SpringSourceRow) {
        let outer = unsafe { Box::from_raw(ptr) };
        let inner = unsafe { Box::from_raw(outer.0) };
        drop(inner);
        drop(outer);
    }

    pub fn into_ptr(self) -> *mut SpringSourceRow {
        Box::into_raw(Box::new(self))
    }
}
