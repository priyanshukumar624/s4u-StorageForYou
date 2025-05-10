use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;


pub async fn init_db_pool() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("❌ Failed to connect to PostgreSQL")
}
