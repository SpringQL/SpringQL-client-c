// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use ::springql_core::api::SpringRow as Row;

use std::{ffi::c_void, mem};
/// Row object from an in memory queue.
#[non_exhaustive]
#[repr(transparent)]
pub struct SpringRow(*mut c_void);

impl SpringRow {
    pub fn new(pipe: Row) -> Self {
        SpringRow(unsafe { mem::transmute(Box::new(pipe)) })
    }

    pub fn as_row(&self) -> &Row {
        unsafe { &*(self.0 as *const Row) }
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