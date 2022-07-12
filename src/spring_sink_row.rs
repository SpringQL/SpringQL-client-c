// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use springql::{Result, SpringSinkRow as RuSpringSinkRow, SpringValue};

/// Row object to pop from an in memory queue.
#[non_exhaustive]
#[derive(Debug)]
pub struct SpringSinkRow(RuSpringSinkRow);

impl From<RuSpringSinkRow> for SpringSinkRow {
    fn from(sink_row: RuSpringSinkRow) -> Self {
        SpringSinkRow(sink_row)
    }
}

impl SpringSinkRow {
    pub(crate) fn get_not_null_by_index<T>(&self, i_col: usize) -> Result<T>
    where
        T: SpringValue,
    {
        self.0.get_not_null_by_index(i_col)
    }

    pub(crate) fn into_ptr(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }
}
