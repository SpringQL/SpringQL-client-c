// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.
use ::springql::SpringPipeline as Pipeline;

use std::{ffi::c_void, mem};
/// Pipeline (dataflow definition) in SpringQL.
#[non_exhaustive]
#[repr(transparent)]
pub struct SpringPipeline(*mut c_void);

impl SpringPipeline {
    pub fn new(pipe: Pipeline) -> Self {
        SpringPipeline(unsafe { mem::transmute(Box::new(pipe)) })
    }

    pub fn as_pipeline(&self) -> &Pipeline {
        unsafe { &*(self.0 as *const Pipeline) }
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
