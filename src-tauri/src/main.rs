// src/main.rs
use std::{net::SocketAddr, sync::Arc};
use tower_http::{cors::CorsLayer, trace::TraceLayer, compression::CompressionLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use dotenvy::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tauri_app_lib::{
    config::{AppConfig, AppState, DbPool},
    create_router,
};

#[derive(OpenApi)]
#[openapi(
    info(title = "ERP Backend API", version = "1.0.0"),
    paths(tauri_app_lib::handlers::health::health_check)
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    info!("Starting ERP Backend Server...");
    
    // Debug: check env loading
    let use_postgres_env = std::env::var("USE_POSTGRES").unwrap_or_default();
    info!("USE_POSTGRES env value: '{}'", use_postgres_env);
    
    let app_config = AppConfig {
        env: std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()),
        jwt_secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret".to_string()),
        cors_origins: std::env::var("CORS_ORIGINS").unwrap_or_else(|_| "*".to_string()),
        use_postgres: std::env::var("USE_POSTGRES").unwrap_or_default() == "true",
        pg_database_url: std::env::var("PG_DATABASE_URL").unwrap_or_default(),
        sqlite_database_url: std::env::var("SQLITE_DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:erp.db?mode=rwc".to_string()),
    };
    
    let app_state = AppState::new(app_config).await;
    
    // Run migrations
// In src/main.rs, update the migration section:

// inside async fn main() 
match app_state.db.as_ref() {
    DbPool::Postgres(pool) => {
        // Try to run migrations, but don't fail if tables exist
        match sqlx::migrate!("./migrations/postgres").run(pool).await {
            Ok(_) => info!("Migrations completed."),
            Err(e) => {
                if e.to_string().contains("already exists") {
                    info!("Tables already exist, skipping migrations...");
                } else {
                    return Err(e.into());
                }
            }
        }
        // Drop problematic FK constraint if it exists
        sqlx::query("ALTER TABLE audit_logs DROP CONSTRAINT IF EXISTS audit_logs_user_id_fkey")
            .execute(pool).await?;
        info!("Ensuring seed data exists...");
        let mut tx = pool.begin().await?;
        // Seed company
        sqlx::query(r#"
            INSERT INTO companies (id, name, email, country, is_active)
            VALUES ('00000000-0000-0000-0000-000000000001', 'Seed Co', 'seed@test.com', 'USA', true)
            ON CONFLICT(id) DO NOTHING
        "#).execute(&mut *tx).await?;
        // Seed role
        sqlx::query(r#"
            INSERT INTO roles (id, company_id, name, role_type, permissions, is_system, is_active)
            VALUES ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'Admin', 'admin', '{}', true, true)
            ON CONFLICT(id) DO NOTHING
        "#).execute(&mut *tx).await?;
        // Seed branch 
        sqlx::query(r#"
            INSERT INTO branches (id, company_id, code, name, address, city, country, is_active)
            VALUES ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'MAIN', 'Main Warehouse', '123 Main St', 'New York', 'USA', true)
            ON CONFLICT(id) DO NOTHING
        "#).execute(&mut *tx).await?;
        // Seed admin user (password: admin123)
        let password_hash = tauri_app_lib::utils::password::hash_password("admin123").unwrap_or_default();
        sqlx::query(r#"
            INSERT INTO users (id, company_id, role_id, username, email, password_hash, first_name, last_name, status)
            VALUES ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'admin', 'admin@erp.com', $1, 'Admin', 'User', 'active')
            ON CONFLICT(id) DO NOTHING
        "#).bind(password_hash).execute(&mut *tx).await?;
        tx.commit().await?;
        info!("Seed data verified.");
    }
    DbPool::Sqlite(pool) => {
// inside src/main.rs -> DbPool::Sqlite(pool) arm
sqlx::migrate!("./migrations/sqlite").run(pool).await?;

info!("Ensuring seed data exists...");
// We use a single transaction to ensure both exist together
let mut tx = pool.begin().await?;

// Use simple INSERT statements without complex columns to ensure they pass
sqlx::query(r#"
    INSERT INTO companies (id, name, code, email, country, is_active)
    VALUES ('00000000-0000-0000-0000-000000000001', 'Seed Co', 'SEED01', 'seed@test.com', 'USA', 1)
    ON CONFLICT(id) DO NOTHING;
"#).execute(&mut *tx).await?;

sqlx::query(r#"
    INSERT INTO roles (id, company_id, name, role_type, permissions, is_system, is_active)
    VALUES ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'Admin', 'admin', '{}', 1, 1)
    ON CONFLICT(id) DO NOTHING;
"#).execute(&mut *tx).await?;

sqlx::query(r#"
    INSERT INTO branches (id, company_id, code, name, address, city, country, is_active)
    VALUES ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'MAIN', 'Main Warehouse', '123 Main St', 'New York', 'USA', 1)
    ON CONFLICT(id) DO NOTHING;
"#).execute(&mut *tx).await?;

tx.commit().await?;
info!("Seed data verified.");
    }
}
    
    let state = Arc::new(app_state);
    let port: u16 = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string()).parse()?;
    
    let app = create_router(state.clone())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());
    
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("Server starting on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await?;
    
    Ok(())
}

async fn shutdown_signal() {
    use tokio::signal;
    tokio::select! {
        _ = signal::ctrl_c() => info!("Shutting down..."),
    }
}