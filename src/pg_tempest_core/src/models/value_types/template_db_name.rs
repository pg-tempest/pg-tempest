use std::{str::FromStr, sync::LazyLock};

use anyhow::anyhow;
use derive_more::{AsRef, Display, Into};
use regex::Regex;

use crate::models::value_types::{pg_identifier::PgIdentifier, template_hash::TemplateHash};

static TEMPLATE_DB_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"TEMPEST_([0-9a-fA-F]{32})_TEMPLATE"#).unwrap());

#[derive(AsRef, Display, Debug, Into, Clone)]
#[display("{pg_identifier}")]
pub struct TemplateDbName {
    #[into]
    pg_identifier: PgIdentifier,
    #[into]
    template_hash: TemplateHash,
}

impl TemplateDbName {
    pub fn new(template_hash: TemplateHash) -> TemplateDbName {
        let identifier = format!("TEMPEST_{template_hash}_TEMPLATE");

        TemplateDbName {
            pg_identifier: PgIdentifier::new(identifier).unwrap(),
            template_hash,
        }
    }
}

impl TryFrom<PgIdentifier> for TemplateDbName {
    type Error = anyhow::Error;

    fn try_from(identifier: PgIdentifier) -> Result<Self, Self::Error> {
        let (_, [template_hash]) = TEMPLATE_DB_NAME_REGEX
            .captures(identifier.as_ref())
            .ok_or(anyhow!("Identifier is not template db name"))?
            .extract();

        let template_hash = TemplateHash::from_str(template_hash)?;

        Ok(TemplateDbName {
            pg_identifier: identifier,
            template_hash,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::value_types::{
        template_db_name::TemplateDbName,
        template_hash::{TEMPLATE_HASH_LENGHT, TemplateHash},
    };

    #[test]
    fn new_template_db_name_formats_correctly() {
        let mut hash = [0u8; TEMPLATE_HASH_LENGHT];
        for (index, byte) in hash.iter_mut().enumerate() {
            *byte = index as u8 + 1;
        }

        let template_hash = TemplateHash::new(hash);
        let template_db_name = TemplateDbName::new(template_hash);

        assert_eq!(
            template_db_name.to_string(),
            "TEMPEST_0102030405060708090A0B0C0D0E0F10_TEMPLATE".to_string()
        );
    }
}
