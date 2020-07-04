use std::env;
use std::sync::Arc;

use anyhow::Result;
use sqlx::postgres::PgPool;

pub async fn connect() -> Result<Arc<PgPool>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    Ok(Arc::new(PgPool::new(&database_url).await?))
}
