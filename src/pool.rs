use sqlx::{Pool, Sqlite, SqlitePool};
use std::{env, error::Error};

pub async fn get_pool() -> Result<Pool<Sqlite>, Box<dyn Error>> {
    let database_url = env::var("DATABASE_URL")?;

    Ok(SqlitePool::connect(&database_url).await?)
}
