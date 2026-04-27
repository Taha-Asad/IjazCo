// src/config.rs
use sqlx::{PgPool, Sqlite, SqlitePool};
use std::sync::Arc;
#[derive(Clone)]
pub enum DbPool {
    Postgres(PgPool),
    Sqlite(SqlitePool),
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub env: String,
    pub jwt_secret: String,
    pub cors_origins: String,
    pub use_postgres: bool,
    pub pg_database_url: String,
    pub sqlite_database_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DbPool>,
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Self {
        let db = if config.use_postgres && !config.pg_database_url.is_empty() {
            println!("Connecting to PostgreSQL...");
            let pool = PgPool::connect(&config.pg_database_url)
                .await
                .expect("Failed to connect to PostgreSQL");
            println!("PostgreSQL connected successfully");
            DbPool::Postgres(pool)
        } else {
            println!("Using SQLite database...");
            
            // Extract path and create directory if needed
            let db_path = config.sqlite_database_url
                .strip_prefix("sqlite:")
                .unwrap_or(&config.sqlite_database_url);
            
            let clean_path = db_path.split('?').next().unwrap_or(db_path);
            
            if let Some(parent) = std::path::Path::new(clean_path).parent() {
                if !parent.as_os_str().is_empty() && !parent.exists() {
                    std::fs::create_dir_all(parent)
                        .expect(&format!("Failed to create directory: {:?}", parent));
                }
            }
            
            let pool = SqlitePool::connect(&config.sqlite_database_url)
                .await
                .expect(&format!("Failed to connect to SQLite at: {}", config.sqlite_database_url));
            println!("SQLite connected successfully");
            
            // Test execute a simple query to verify connection works (also verifies FK is enabled via after_connect)
            let _test = sqlx::query("SELECT 1").fetch_one(&pool).await.unwrap();
            
            // Verify foreign keys is enabled
            let row: (i32,) = sqlx::query_as::<Sqlite, _>("PRAGMA foreign_keys").fetch_one(&pool).await.unwrap();
            println!("Foreign keys enabled: {}", row.0);
            
            DbPool::Sqlite(pool)
            
        };

        Self {
            db: Arc::new(db),
            config: Arc::new(config),
        }
    }

    pub fn pg(&self) -> Option<&PgPool> {
        match self.db.as_ref() {
            DbPool::Postgres(pool) => Some(&pool),
            _ => None,
        }
    }

    pub fn sqlite(&self) -> Option<&SqlitePool> {
        match self.db.as_ref() {
            DbPool::Sqlite(pool) => Some(&pool),
            _ => None,
        }
    }
}