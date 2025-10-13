use sqlx::PgPool;

use crate::{
    db_queries::{create_db::create_db, drop_template_db::drop_template_db, get_dbs::get_dbs},
    models::value_types::template_db_name::TemplateDbName,
};

pub async fn recreate_template_db(
    pg_pool: &PgPool,
    template_db_name: &TemplateDbName,
) -> anyhow::Result<()> {
    let databases = get_dbs(pg_pool).await?;
    let template_db_exists = databases
        .iter()
        .find(|x| x.name == *template_db_name.as_ref())
        .is_some();

    if template_db_exists {
        drop_template_db(pg_pool, template_db_name.as_ref()).await?;
    }

    create_db(pg_pool, &template_db_name.as_ref(), true).await?;

    Ok(())
}
