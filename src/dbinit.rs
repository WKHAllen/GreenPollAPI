use crate::util::DBPool;

/// Initialize the database tables
/// 
/// # Arguments
/// 
/// * `pool` - The database pool
pub async fn init_db(pool: &DBPool) -> Result<(), sqlx::Error> {
    sqlx::query_file!("sql/init/user.sql").fetch_all(pool).await?;
    sqlx::query_file!("sql/init/poll.sql").fetch_all(pool).await?;
    sqlx::query_file!("sql/init/poll_option.sql").fetch_all(pool).await?;
    sqlx::query_file!("sql/init/poll_vote.sql").fetch_all(pool).await?;
    sqlx::query_file!("sql/init/session.sql").fetch_all(pool).await?;
    sqlx::query_file!("sql/init/verify.sql").fetch_all(pool).await?;
    sqlx::query_file!("sql/init/password_reset.sql").fetch_all(pool).await?;

    Ok(())
}
