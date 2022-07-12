// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use ::springql::SpringSourceRow as RuSpringSourceRow;

/// Row object to push into an in memory queue.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct SpringSourceRow(RuSpringSourceRow);

impl From<RuSpringSourceRow> for SpringSourceRow {
    fn from(source_row: RuSpringSourceRow) -> Self {
        SpringSourceRow(source_row)
    }
}
impl From<SpringSourceRow> for RuSpringSourceRow {
    fn from(source_row: SpringSourceRow) -> Self {
        source_row.0
    }
}

impl SpringSourceRow {
    pub fn into_ptr(self) -> *mut SpringSourceRow {
        Box::into_raw(Box::new(self))
    }
}
