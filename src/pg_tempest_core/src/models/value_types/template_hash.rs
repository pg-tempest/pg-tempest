use derive_more::Display;
use hex::FromHexError;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, str::FromStr};

pub const TEMPLATE_HASH_LENGTH: usize = 16;
pub const TEMPLATE_HASH_LENGTH_IN_HEX: usize = 32;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, Deserialize, Serialize, Default)]
#[display("{self:?}")]
#[serde(try_from = "&str")]
#[serde(into = "Box<str>")]
pub struct TemplateHash {
    value: [u8; TEMPLATE_HASH_LENGTH],
}

impl TemplateHash {
    pub fn new(value: [u8; TEMPLATE_HASH_LENGTH]) -> TemplateHash {
        TemplateHash { value }
    }
}

impl Debug for TemplateHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = hex::encode_upper(&self.value);
        f.write_str(str.as_str())
    }
}

impl TryFrom<&str> for TemplateHash {
    type Error = FromHexError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        TemplateHash::from_str(s)
    }
}

impl From<TemplateHash> for Box<str> {
    fn from(hash: TemplateHash) -> Self {
        hash.to_string().into()
    }
}

impl From<TemplateHash> for String {
    fn from(hash: TemplateHash) -> Self {
        hash.to_string()
    }
}

impl FromStr for TemplateHash {
    type Err = FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hash = [0u8; TEMPLATE_HASH_LENGTH];
        hex::decode_to_slice(s, &mut hash)?;

        Ok(TemplateHash::new(hash))
    }
}

#[cfg(test)]
mod tests {
    use crate::models::value_types::template_hash::TemplateHash;

    #[test]
    fn template_hash_formats_as_upper_hex() {
        let template_hash = TemplateHash::new([
            01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16,
        ]);

        assert_eq!(
            template_hash.to_string(),
            String::from("0102030405060708090A0B0C0D0E0F10")
        )
    }
}
