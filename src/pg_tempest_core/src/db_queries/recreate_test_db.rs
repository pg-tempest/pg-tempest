use sqlx::PgPool;

use crate::{
    db_queries::{
        create_db_with_template::create_db_with_template, drop_db::drop_db, get_dbs::get_dbs,
    },
    models::value_types::{template_db_name::TemplateDbName, test_db_name::TestDbName},
};

pub async fn recreate_test_db(
    pg_pool: &PgPool,
    test_db_name: &TestDbName,
    template_db_name: &TemplateDbName,
) -> anyhow::Result<()> {
    let dbs = get_dbs(pg_pool).await?;
    let test_db_exists = dbs
        .iter()
        .find(|x| x.name == *test_db_name.as_ref())
        .is_some();

    if test_db_exists {
        drop_db(pg_pool, test_db_name.as_ref()).await?;
    }

    create_db_with_template(pg_pool, test_db_name.as_ref(), template_db_name.as_ref()).await?;

    Ok(())
}
