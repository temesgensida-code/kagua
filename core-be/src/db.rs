use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::info;

/// Initialize the PostgreSQL connection pool and run all SQL migrations.
pub async fn init_db(database_url: &str) -> PgPool {
    info!("Connecting to PostgreSQL database...");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("Failed to connect to PostgreSQL. Check DATABASE_URL in .env");

    info!("Running database migrations...");
    sqlx::raw_sql(include_str!("../migrations/001_create_users.sql"))
        .execute(&pool)
        .await
        .expect("Migration 001 failed");
    sqlx::raw_sql(include_str!("../migrations/002_create_refresh_tokens.sql"))
        .execute(&pool)
        .await
        .expect("Migration 002 failed");
    sqlx::raw_sql(include_str!("../migrations/003_create_email_tokens.sql"))
        .execute(&pool)
        .await
        .expect("Migration 003 failed");

    info!("Database ready.");
    pool
}
