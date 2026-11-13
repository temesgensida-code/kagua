use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::info;

/// Initialize the PostgreSQL connection pool and run all SQL migrations.
pub async fn init_db(database_url: &str) -> PgPool {
    info!("Connecting to PostgreSQL database with exponential retry...");
    let mut attempts = 0;
    let max_attempts = 5;
    let mut delay = std::time::Duration::from_secs(1);

    let pool = loop {
        attempts += 1;
        match PgPoolOptions::new()
            .max_connections(10)
            .min_connections(2)
            .acquire_timeout(std::time::Duration::from_secs(15))
            .idle_timeout(std::time::Duration::from_secs(300))
            .max_lifetime(std::time::Duration::from_secs(1800))
            .connect(database_url)
            .await
        {
            Ok(p) => {
                info!("PostgreSQL connected successfully on attempt {}", attempts);
                break p;
            }
            Err(e) if attempts < max_attempts => {
                tracing::warn!(
                    "DB connect attempt {}/{} failed: {}. Retrying in {:?}...",
                    attempts,
                    max_attempts,
                    e,
                    delay
                );
                tokio::time::sleep(delay).await;
                delay *= 2;
            }
            Err(e) => {
                panic!(
                    "Fatal: Failed to connect to PostgreSQL after {} attempts: {}. Check DATABASE_URL in .env",
                    attempts, e
                );
            }
        }
    };

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
    sqlx::raw_sql(include_str!("../migrations/004_create_audit_reports.sql"))
        .execute(&pool)
        .await
        .expect("Migration 004 failed");

    info!("Seeding test user...");
    if let Ok(hash) = crate::auth::password::hash_password("@Te1to2ge3si4") {
        let seed_res = sqlx::query(
            "INSERT INTO users (email, password_hash, is_verified) \
             VALUES ('test@gmail.com', $1, TRUE) \
             ON CONFLICT (email) DO UPDATE SET password_hash = $1, is_verified = TRUE"
        )
        .bind(&hash)
        .execute(&pool)
        .await;

        if let Err(e) = seed_res {
            tracing::warn!("Failed to seed test user: {}", e);
        } else {
            info!("Test user test@gmail.com initialized.");
        }
    }

    info!("Database ready.");
    pool
}

