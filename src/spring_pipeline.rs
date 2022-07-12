// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use ::springql::SpringPipeline as RuSpringPipeline;

/// Pipeline (dataflow definition) in SpringQL.
#[non_exhaustive]
#[derive(Debug)]
pub struct SpringPipeline(RuSpringPipeline);

impl From<RuSpringPipeline> for SpringPipeline {
    fn from(pipeline: RuSpringPipeline) -> Self {
        SpringPipeline(pipeline)
    }
}

impl AsRef<RuSpringPipeline> for SpringPipeline {
    fn as_ref(&self) -> &RuSpringPipeline {
        &self.0
    }
}

impl SpringPipeline {
    pub(crate) fn into_ptr(self) -> *mut SpringPipeline {
        Box::into_raw(Box::new(self))
    }
}
