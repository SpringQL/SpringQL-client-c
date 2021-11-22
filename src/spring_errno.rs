use springql_core::error::SpringError;

use crate::spring_last_err::LastError;

/// Errno (error number) to be returned erroneous functions.
///
/// See springql_core::api::error::SpringError for details of each error reason.
#[non_exhaustive]
#[repr(C)]
pub enum SpringErrno {
    Ok = 0,

    /// Panic
    Unknown = -1,

    ForeignIo = -2,
    ForeignSourceTimeout = -3,
    InputTimeout = -4,
    SpringQlCoreIo = -5,
    ThreadPoisoned = -6,
    InvalidOption = -7,
    InvalidFormat = -8,
    Unavailable = -9,
    Sql = -10,

    /// Insufficient buffer size
    CInsufficient = -126,
    /// Invalid null pointer
    CNull = -127,
}

impl From<&SpringError> for SpringErrno {
    fn from(e: &SpringError) -> Self {
        match e {
            SpringError::ForeignIo { .. } => SpringErrno::ForeignIo,
            SpringError::ForeignSourceTimeout { .. } => SpringErrno::ForeignSourceTimeout,
            SpringError::InputTimeout { .. } => SpringErrno::InputTimeout,
            SpringError::SpringQlCoreIo(_) => SpringErrno::SpringQlCoreIo,
            SpringError::ThreadPoisoned(_) => SpringErrno::ThreadPoisoned,
            SpringError::InvalidOption { .. } => SpringErrno::InvalidOption,
            SpringError::InvalidFormat { .. } => SpringErrno::InvalidFormat,
            SpringError::Unavailable { .. } => SpringErrno::Unavailable,
            SpringError::Sql(_) => SpringErrno::Sql,
        }
    }
}

impl From<&LastError> for SpringErrno {
    fn from(e: &LastError) -> Self {
        match e {
            LastError::SpringErr(e) => e.into(),
            LastError::UnwindErr(_) => SpringErrno::Unknown,
        }
    }
}
