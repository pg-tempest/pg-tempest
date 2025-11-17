use derive_more::{AsRef, Display, Into};
use regex::Regex;
use std::{str::FromStr, sync::LazyLock};

use crate::models::value_types::{
    pg_identifier::PgIdentifier, template_hash::TemplateHash, test_db_id::TestDbId,
};

static TEMPLATE_DB_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"TEMPEST_([0-9a-fA-F]{32})_TEST_DB([0-9a-fA-F]{4})"#).unwrap());

#[derive(AsRef, Display, Debug, Into, Clone)]
#[display("{pg_identifier}")]
pub struct TestDbName {
    #[as_ref]
    #[into]
    pg_identifier: PgIdentifier,
    #[as_ref]
    template_hash: TemplateHash,
    #[as_ref]
    test_db_id: TestDbId,
}

impl TestDbName {
    pub fn new(template_hash: TemplateHash, test_db_id: TestDbId) -> TestDbName {
        let identifier = format!("TEMPEST_{template_hash}_TEST_DB_{test_db_id}");

        TestDbName {
            pg_identifier: PgIdentifier::new(identifier).unwrap(),
            template_hash,
            test_db_id,
        }
    }
}

impl TryFrom<PgIdentifier> for TestDbName {
    type Error = String;

    fn try_from(identifier: PgIdentifier) -> Result<Self, Self::Error> {
        let (_, [template_hash, test_db_id]) = TEMPLATE_DB_NAME_REGEX
            .captures(identifier.as_ref())
            .ok_or("Identifier is not test db name")?
            .extract();

        // Format of a template hash and a test db id is validated by TEMPLATE_DB_NAME_REGEX
        let template_hash = TemplateHash::from_str(template_hash).unwrap();
        let test_db_id = TestDbId::from_str(test_db_id).unwrap();

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
        template_hash::{TEMPLATE_HASH_LENGTH, TemplateHash},
        test_db_id::TestDbId,
        test_db_name::TestDbName,
    };

    #[test]
    fn new_test_db_name_formats_correctly() {
        let mut hash = [0u8; TEMPLATE_HASH_LENGTH];
        for (index, byte) in hash.iter_mut().enumerate() {
            *byte = index as u8 + 1;
        }

        let template_hash = TemplateHash::new(hash);
        let test_db_id = TestDbId::new(0x0100);
        let test_db_name = TestDbName::new(template_hash, test_db_id);

        assert_eq!(
            test_db_name.to_string(),
            "TEMPEST_0102030405060708090A0B0C0D0E0F10_TEST_DB_0100".to_string()
        );
    }
}
