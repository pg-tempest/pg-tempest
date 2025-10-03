use derive_more::{AsRef, Display};
use regex::Regex;

#[derive(Debug, AsRef, Display, Clone)]
#[display("\"{}\"", self.value)]
pub struct PgIdentifier {
    value: String,
}

impl PgIdentifier {
    pub fn new(value: impl Into<String>) -> Result<PgIdentifier, PgIdentifierParseError> {
        let value = value.into();
        let regex = Regex::new("[a-zA-Z0-9\\-]?").unwrap();
        if regex.is_match(value.as_str()) {
            Ok(PgIdentifier { value })
        } else {
            Err(PgIdentifierParseError { value })
        }
    }
}

#[derive(Debug, Display)]
#[display("{self:?}")]
pub struct PgIdentifierParseError {
    value: String,
}
