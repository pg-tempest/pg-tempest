use extension_trait::extension_trait;

use crate::{models::value_types::pg_identifier::PgIdentifier, pg_client::PgClient};

#[extension_trait]
#[allow(async_fn_in_trait)]
pub impl PgClientExtensions for dyn PgClient {
    async fn drop_template_db(&self, db_name: PgIdentifier) -> anyhow::Result<()> {
        self.alter_db_is_template(db_name.clone(), false).await?;
        self.drop_db(db_name).await?;

        Ok(())
    }

    async fn recreate_db(
        &self,
        db_name: PgIdentifier,
        template_db_name: Option<PgIdentifier>,
        is_template: bool,
    ) -> anyhow::Result<()> {
        let dbs = self.get_dbs().await?;
        let db_exists = dbs.iter().find(|x| x.name == db_name).is_some();

        if db_exists {
            if is_template {
                self.alter_db_is_template(db_name.clone(), false).await?;
            }
            self.drop_db(db_name.clone()).await?;
        }

        self.create_db(db_name, template_db_name, is_template)
            .await?;

        Ok(())
    }
}
