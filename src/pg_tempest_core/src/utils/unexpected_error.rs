use derive_more::{Debug as DebugV2, Display};
use std::error::Error;

#[derive(DebugV2, Display)]
#[debug("{value}")]
#[display("{value}")]
pub struct UnexpectedError {
    value: String,
}

impl UnexpectedError {
    pub fn new(value: String) -> UnexpectedError {
        UnexpectedError { value }
    }
}

impl<T: Into<Box<dyn Error>>> From<T> for UnexpectedError {
    fn from(value: T) -> Self {
        UnexpectedError {
            value: value.into().to_string(),
        }
    }
}