use extension_trait::extension_trait;

use crate::{
    models::value_types::{
        pg_identifier::PgIdentifier, template_db_name::TemplateDbName, test_db_name::TestDbName,
    },
    pg_client::PgClient,
};

#[extension_trait]
#[allow(async_fn_in_trait)]
pub impl PgClientExtensions for dyn PgClient {
    async fn drop_template_db(&self, db_name: &PgIdentifier) -> anyhow::Result<()> {
        self.alter_db_is_template(db_name, false).await?;
        self.drop_db(db_name).await?;

        Ok(())
    }

    async fn recreate_template_db(&self, template_db_name: &TemplateDbName) -> anyhow::Result<()> {
        let databases = self.get_dbs().await?;
        let template_db_exists = databases
            .iter()
            .find(|x| x.name == *template_db_name.as_ref())
            .is_some();

        if template_db_exists {
            self.drop_template_db(template_db_name.as_ref()).await?;
        }

        self.create_db(&template_db_name.as_ref(), true).await?;

        Ok(())
    }

    async fn recreate_test_db(
        &self,
        test_db_name: &TestDbName,
        template_db_name: &TemplateDbName,
    ) -> anyhow::Result<()> {
        let dbs = self.get_dbs().await?;
        let test_db_exists = dbs
            .iter()
            .find(|x| x.name == *test_db_name.as_ref())
            .is_some();

        if test_db_exists {
            self.drop_db(test_db_name.as_ref()).await?;
        }

        self.create_db_with_template(test_db_name.as_ref(), template_db_name.as_ref())
            .await?;

        Ok(())
    }
}
