use crate::util::DBPool;

pub async fn init_db(pool: &DBPool) -> Result<(), sqlx::Error> {
    sqlx::query_file!("sql/init/user.sql").fetch_all(pool).await?;
    Ok(())
}
