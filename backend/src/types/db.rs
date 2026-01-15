use std::env;

use once_cell::sync::OnceCell;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub static DB: OnceCell<PgPool> = OnceCell::new();
pub async fn init_db() {
    let url = env::var("DATABASE_URL").expect("No DATABASE_URL env var.");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await
        .expect("Failed to connect to database.");
    let migrator = sqlx::migrate!();
    match migrator.run(&pool).await {
        Ok(_) => {}
        Err(sqlx::migrate::MigrateError::VersionMismatch(version)) => {
            println!(
                "Migration mismatch for version {}. Attempting to repair checksum...",
                version
            );
            if let Some(m) = migrator.migrations.iter().find(|m| m.version == version) {
                let checksum = &m.checksum;
                sqlx::query("UPDATE _sqlx_migrations SET checksum = $1 WHERE version = $2")
                    .bind(checksum.as_ref())
                    .bind(version)
                    .execute(&pool)
                    .await
                    .expect("Failed to update migration checksum");

                println!("Checksum repaired. Retrying migration...");
                migrator
                    .run(&pool)
                    .await
                    .expect("Migration failed after repair");
            } else {
                panic!(
                    "Migration failed: VersionMismatch({}) but migration not found in local files.",
                    version
                );
            }
        }
        Err(e) => panic!("Migration failed: {:?}", e),
    }
    DB.set(pool).expect("DB already initialized.");
}
pub fn db() -> &'static PgPool {
    DB.get()
        .expect("Database connection is not initialized. Call init_db frist.")
}
