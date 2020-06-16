use std::env;

use anyhow::Result;
use sqlx::postgres::PgPool;

pub async fn connect() -> Result<PgPool> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    Ok(PgPool::new(&database_url).await?)
}
