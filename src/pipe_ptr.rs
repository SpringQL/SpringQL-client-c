use ::springql_core::api::SpringPipelineHL;

use std::{ffi::c_void, mem};
/// Pipeline (dataflow definition) in SpringQL.
#[non_exhaustive]
#[repr(transparent)]
pub struct SpringPipeline(*mut c_void);

impl SpringPipeline {
    pub fn new(pipe: SpringPipelineHL) -> Self {
        SpringPipeline(unsafe { mem::transmute(Box::new(pipe)) })
    }

    pub fn pipe(&self) -> &SpringPipelineHL {
        unsafe { &*(self.0 as *const SpringPipelineHL) }
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
