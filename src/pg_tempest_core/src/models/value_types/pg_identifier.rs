use std::{str::FromStr, sync::LazyLock};

use derive_more::{AsRef, Debug, Display, Into};
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

static PG_IDENTIFIER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[\\w_]+[\\w\\d_$]?$").unwrap());

#[derive(
    Debug, AsRef, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Into, Serialize, Deserialize,
)]
#[display("{value}")]
#[debug("{value}")]
#[serde(try_from = "&str")]
#[serde(into = "Box<str>")]
pub struct PgIdentifier {
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

impl FromStr for PgIdentifier {
    type Err = PgIdentifierParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PgIdentifier::new(s)
    }
}

impl From<PgIdentifier> for String {
    fn from(value: PgIdentifier) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for PgIdentifier {
    type Error = PgIdentifierParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        PgIdentifier::new(s)
    }
}

#[derive(Debug, Display, Error)]
#[display("{self:?}")]
pub struct PgIdentifierParseError {
    pub value: Box<str>,
}
