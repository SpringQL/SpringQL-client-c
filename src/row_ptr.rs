use ::springql_core::api::high_level_rs::SpringRowHL;

use std::{ffi::c_void, mem};
/// Row object from an in memory queue.
#[non_exhaustive]
#[repr(transparent)]
pub struct SpringRow(*mut c_void);

impl SpringRow {
    pub fn new(pipe: SpringRowHL) -> Self {
        SpringRow(unsafe { mem::transmute(Box::new(pipe)) })
    }

    pub fn row(&self) -> &SpringRowHL {
        unsafe { &*(self.0 as *const SpringRowHL) }
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
