use std::fmt;

/// Error thrown automatically when the provided call method triggers an uwinding panic.
#[derive(Debug, Clone)]
pub struct CallError {
    message: String,
}

impl CallError {
    pub fn new<T: AsRef<str>>(message: T) -> CallError {
        CallError {
            message: String::from(message.as_ref()),
        }
    }
}

impl fmt::Display for CallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CallError {}

#[doc(hidden)]
pub type CallResult<T> = Result<T, CallError>;
