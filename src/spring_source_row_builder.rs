// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use ::springql::SpringSourceRowBuilder as SourceRowBuilder;

use std::{ffi::c_void, mem};

/// Builder of SpringSourceRow
#[non_exhaustive]
#[repr(transparent)]
pub struct SpringSourceRowBuilder(*mut c_void);

impl SpringSourceRowBuilder {
    pub fn new(underlying: SourceRowBuilder) -> Self {
        SpringSourceRowBuilder(unsafe { mem::transmute(Box::new(underlying)) })
    }

    pub fn to_row_builder(&self) -> SourceRowBuilder {
        unsafe { &*(self.0 as *const SourceRowBuilder) }.clone()
    }

    pub fn drop(ptr: *mut SpringSourceRowBuilder) {
        let outer = unsafe { Box::from_raw(ptr) };
        let inner = unsafe { Box::from_raw(outer.0) };
        drop(inner);
        drop(outer);
    }

    pub fn into_ptr(self) -> *mut SpringSourceRowBuilder {
        Box::into_raw(Box::new(self))
    }
}

impl Default for SpringSourceRowBuilder {
    fn default() -> Self {
        Self::new(SourceRowBuilder::default())
    }
}
