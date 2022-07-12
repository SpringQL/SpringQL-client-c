// This file is part of https://github.com/SpringQL/SpringQL-client-c which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.
use ::springql::{Result, SpringConfig as RuSpringConfig};

/// Configuration.
#[non_exhaustive]
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct SpringConfig(RuSpringConfig);

impl AsRef<RuSpringConfig> for SpringConfig {
    fn as_ref(&self) -> &RuSpringConfig {
        &self.0
    }
}

impl SpringConfig {
    pub(crate) fn from_toml(toml: &str) -> Result<Self> {
        let config = RuSpringConfig::from_toml(toml)?;
        Ok(Self(config))
    }

    pub(crate) fn into_ptr(self) -> *mut SpringConfig {
        Box::into_raw(Box::new(self))
    }
}
