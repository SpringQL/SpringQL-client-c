// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use ::springql::SpringSinkRow as SinkRow;

use std::{ffi::c_void, mem};
/// Row object from an in memory queue.
#[non_exhaustive]
#[repr(transparent)]
pub struct SpringSinkRow(*mut c_void);

impl SpringSinkRow {
    pub fn new(sink_row: SinkRow) -> Self {
        SpringSinkRow(unsafe { mem::transmute(Box::new(sink_row)) })
    }

    pub fn as_row(&self) -> &SinkRow {
        unsafe { &*(self.0 as *const SinkRow) }
    }

    pub fn drop(ptr: *mut SpringSinkRow) {
        let outer = unsafe { Box::from_raw(ptr) };
        let inner = unsafe { Box::from_raw(outer.0) };
        drop(inner);
        drop(outer);
    }

    pub fn into_ptr(self) -> *mut SpringSinkRow {
        Box::into_raw(Box::new(self))
    }
}
