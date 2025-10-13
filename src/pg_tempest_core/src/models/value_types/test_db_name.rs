use std::{str::FromStr, sync::LazyLock};

use anyhow::anyhow;
use derive_more::{AsRef, Debug, Display};
use regex::Regex;

use crate::models::value_types::{
    pg_identifier::PgIdentifier, template_hash::TemplateHash, test_db_id::TestDbId,
};

static TEMPLATE_DB_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"TEMPEST_([0-9a-fA-F]{32})_TEST_DB([0-9a-fA-F]{4})"#).unwrap());

#[derive(AsRef, Display, Debug)]
#[display("{pg_identifier}")]
pub struct TestDbName {
    #[as_ref]
    pg_identifier: PgIdentifier,
    #[as_ref]
    template_hash: TemplateHash,
    #[as_ref]
    test_db_id: TestDbId,
}

impl TestDbName {
    pub fn new(template_hash: TemplateHash, test_db_id: TestDbId) -> TestDbName {
        let identifier = format!("TEMPEST_{template_hash}_TEST_DB_{test_db_id:04X}");

        TestDbName {
            pg_identifier: PgIdentifier::new(identifier).unwrap(),
            template_hash: template_hash,
            test_db_id: test_db_id,
        }
    }
}

impl TryFrom<PgIdentifier> for TestDbName {
    type Error = anyhow::Error;

    fn try_from(identifier: PgIdentifier) -> Result<Self, Self::Error> {
        let (_, [template_hash, test_db_id]) = TEMPLATE_DB_NAME_REGEX
            .captures(identifier.as_ref())
            .ok_or(anyhow!("Identifier is not test db name"))?
            .extract();

        let template_hash = TemplateHash::from_str(template_hash)?;
        let test_db_id = u16::from_str_radix(test_db_id, 16)?;

        Ok(TestDbName {
            pg_identifier: identifier,
            template_hash,
            test_db_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::value_types::{
        template_hash::{TEMPLATE_HASH_LENGHT, TemplateHash},
        test_db_name::TestDbName,
    };

    #[test]
    fn new_test_db_name_formats_correctly() {
        let mut hash = [0u8; TEMPLATE_HASH_LENGHT];
        for (index, byte) in hash.iter_mut().enumerate() {
            *byte = index as u8 + 1;
        }

        let template_hash = TemplateHash::new(hash);
        let test_db_name = TestDbName::new(template_hash, 0x0100);

        assert_eq!(
            test_db_name.to_string(),
            "TEMPEST_0102030405060708090A0B0C0D0E0F10_TEST_DB_0100".to_string()
        );
    }
}
