use derive_more::{
    AsRef,
    // Alias is used as a work-around to
    // https://youtrack.jetbrains.com/issue/RUST-9732/Derive-macros-have-wrong-priorities-in-name-resolution
    Debug as DebugV2,
    Display,
    Into,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{str::FromStr, sync::LazyLock};
use thiserror::Error;

static PG_IDENTIFIER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[\\w_]+[\\w\\d_$]?$").unwrap());

#[derive(
    DebugV2, AsRef, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Into, Serialize, Deserialize,
)]
#[display("{value}")]
#[debug("{value}")]
#[serde(try_from = "Arc<str>")]
#[serde(into = "Arc<str>")]
pub struct PgIdentifier {
    value: Arc<str>,
}

impl PgIdentifier {
    pub fn new(value: impl Into<Arc<str>>) -> Result<PgIdentifier, PgIdentifierParseError> {
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

impl TryFrom<Arc<str>> for PgIdentifier {
    type Error = PgIdentifierParseError;

    fn try_from(s: Arc<str>) -> Result<Self, Self::Error> {
        PgIdentifier::new(s)
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
    pub value: Arc<str>,
}
