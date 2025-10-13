use std::sync::LazyLock;

use derive_more::{AsRef, Display, Into};
use regex::Regex;
use thiserror::Error;

static PG_IDENTIFIER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[\\w_]+[\\w\\d_$]?$").unwrap());

#[derive(Debug, AsRef, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Into)]
#[display("{value}")]
pub struct PgIdentifier {
    #[into]
    value: Box<str>,
}

impl PgIdentifier {
    pub fn new(value: impl Into<Box<str>>) -> Result<PgIdentifier, PgIdentifierParseError> {
        let value = value.into();
        if PG_IDENTIFIER_REGEX.is_match(&value) {
            Ok(PgIdentifier { value })
        } else {
            Err(PgIdentifierParseError { value })
        }
    }
}

#[derive(Debug, Display, Error)]
#[display("{self:?}")]
pub struct PgIdentifierParseError {
    value: Box<str>,
}
