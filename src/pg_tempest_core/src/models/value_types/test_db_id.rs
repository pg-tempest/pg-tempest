use std::{fmt::Debug, str::FromStr};

use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, Deserialize, Serialize, Default)]
#[display("{self:?}")]
#[serde(try_from = "&str")]
#[serde(into = "Box<str>")]
pub struct TestDbId {
    value: u16,
}

impl TestDbId {
    pub fn new(value: u16) -> TestDbId {
        TestDbId { value }
    }
}

impl Debug for TestDbId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04X}", self.value)
    }
}

impl TryFrom<&str> for TestDbId {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        TestDbId::from_str(s)
    }
}

impl From<TestDbId> for Box<str> {
    fn from(hash: TestDbId) -> Self {
        hash.to_string().into()
    }
}

impl From<TestDbId> for String {
    fn from(hash: TestDbId) -> Self {
        hash.to_string()
    }
}

impl FromStr for TestDbId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u16 = u16::from_str_radix(s, 16)?;

        Ok(TestDbId::new(value))
    }
}
