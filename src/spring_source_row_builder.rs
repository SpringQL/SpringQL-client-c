// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use ::springql::SpringSourceRowBuilder as RuSpringSourceRowBuilder;

/// Builder of SpringSourceRow
#[non_exhaustive]
#[derive(PartialEq, Debug, Default)]
pub struct SpringSourceRowBuilder(RuSpringSourceRowBuilder);

impl From<RuSpringSourceRowBuilder> for SpringSourceRowBuilder {
    fn from(builder: RuSpringSourceRowBuilder) -> Self {
        SpringSourceRowBuilder(builder)
    }
}
impl From<SpringSourceRowBuilder> for RuSpringSourceRowBuilder {
    fn from(builder: SpringSourceRowBuilder) -> Self {
        builder.0
    }
}

impl SpringSourceRowBuilder {
    pub fn into_ptr(self) -> *mut SpringSourceRowBuilder {
        Box::into_raw(Box::new(self))
    }
}
