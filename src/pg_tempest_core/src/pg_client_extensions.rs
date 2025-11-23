use crate::pg_client::{CreateDbError, DropDbError};
use crate::utils::errors::{BoxDynError, ErrorExt};
use crate::{models::value_types::pg_identifier::PgIdentifier, pg_client::PgClient};
use derive_more::{Debug as DebugV2, Display};
use extension_trait::extension_trait;
use thiserror::Error;

#[extension_trait]
#[allow(async_fn_in_trait)]
pub impl PgClientExtensions for dyn PgClient {
    async fn drop_template_db(&self, db_name: PgIdentifier) -> Result<(), BoxDynError> {
        self.alter_db_is_template(db_name.clone(), false).await?;
        self.drop_db(db_name).await?;

        Ok(())
    }

    async fn recreate_db(
        &self,
        db_name: PgIdentifier,
        template_db_name: Option<PgIdentifier>,
    ) -> Result<(), RecreateDbError> {
        match self.drop_db(db_name.clone()).await {
            Ok(_) => {}
            Err(DropDbError::DbIsTemplate { db_name }) => {
                return Err(RecreateDbError::DbIsTemplate { db_name });
            }
            Err(DropDbError::DbDoesNotExist { .. }) => {}
            Err(err) => return Err(RecreateDbError::Unexpected(err.into())),
        };

        self.create_db(db_name, template_db_name, false)
            .await
            .box_err()?;

        Ok(())
    }

    async fn recreate_template_db(
        &self,
        template_db_name: PgIdentifier,
        parent_template_db_name: Option<PgIdentifier>,
    ) -> Result<(), RecreateTemplateDbError> {
        match self.drop_db(template_db_name.clone()).await {
            Ok(_) => {}
            Err(DropDbError::DbIsTemplate { .. }) => {
                self.alter_db_is_template(template_db_name.clone(), false)
                    .await
                    .box_err()?;

                self.drop_db(template_db_name.clone()).await.box_err()?;
            }
            Err(DropDbError::DbDoesNotExist { .. }) => {}
            Err(err) => {
                return Err(RecreateTemplateDbError::Unexpected(err.into()));
            }
        };

        let create_db_result = self
            .create_db(template_db_name, parent_template_db_name, true)
            .await;

        match create_db_result {
            Ok(_) => Ok(()),
            Err(CreateDbError::TemplateDbDoesNotExist { template_db_name }) => {
                Err(RecreateTemplateDbError::ParentTemplateDbDoesNotExist {
                    parent_template_db_name: template_db_name,
                })
            }
            Err(err) => Err(RecreateTemplateDbError::Unexpected(err.into())),
        }
    }
}

#[derive(DebugV2, Display, Error)]
#[display("RecreateDbError::{self:?}")]
pub enum RecreateDbError {
    DbIsTemplate {
        db_name: PgIdentifier,
    },
    TemplateDbWasNotFound {
        template_db_name: PgIdentifier,
    },
    Unexpected(
        #[from]
        #[debug("{_0}")]
        BoxDynError,
    ),
}

#[derive(Error, DebugV2, Display)]
#[display("RecreateTemplateDbError::{self:?}")]
pub enum RecreateTemplateDbError {
    ParentTemplateDbDoesNotExist {
        parent_template_db_name: PgIdentifier,
    },
    Unexpected(
        #[from]
        #[debug("{_0}")]
        BoxDynError,
    ),
}
