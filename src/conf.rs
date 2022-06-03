use ::springql_core::api::SpringConfig as RawSpringConfig;
use std::{ffi::c_void, mem};
/// Configuration.
#[non_exhaustive]
#[repr(transparent)]
pub struct SpringConfig(*mut c_void);

impl SpringConfig {
    pub fn new(config: RawSpringConfig) -> Self {
        SpringConfig(unsafe { mem::transmute(Box::new(config)) })
    }

    pub fn llconf(&self) -> &RawSpringConfig {
        unsafe { &*(self.0 as *const RawSpringConfig) }
    }

    pub fn drop(ptr: *mut SpringConfig) {
        let outer = unsafe { Box::from_raw(ptr) };
        let inner = unsafe { Box::from_raw(outer.0) };
        drop(inner);
        drop(outer);
    }

    pub fn into_ptr(self) -> *mut SpringConfig {
        Box::into_raw(Box::new(self))
    }
}
