use sqlx::PgPool;

use crate::{
    db_queries::{alter_db_is_template::alter_db_is_template, drop_db::drop_db},
    models::value_types::pg_identifier::PgIdentifier,
};

pub async fn drop_template_db(pg_pool: &PgPool, db_name: &PgIdentifier) -> anyhow::Result<()> {
    alter_db_is_template(pg_pool, db_name, false).await?;
    drop_db(pg_pool, db_name).await?;

    Ok(())
}
