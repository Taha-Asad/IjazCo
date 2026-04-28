// tests/common/mod.rs
// Common test utilities and setup functions
// Provides shared test infrastructure for all test modules

use sqlx::{PgPool, SqlitePool, Sqlite};
use sqlx::types::Decimal;
use uuid::Uuid;
use std::sync::Arc;

// Re-export commonly used test dependencies from crate
pub use crate::{
    models::user::{User, UserRole, UserStatus, CreateUserRequest},
    models::inventory::{InventoryItem, CreateItemRequest},
    models::sales::{SalesInvoice, CreateSalesInvoiceRequest},
    utils::jwt::{generate_jwt, TokenType},
    config::{AppState, DbPool, AppConfig},
};

// ===== TEST DATABASE SETUP =====
// Creates an in-memory SQLite database for testing
// Each test gets a fresh, isolated database instance
pub async fn setup_test_db() -> SqlitePool {
    // Create in-memory SQLite database (unique per test)
    // :memory: creates a new database for each connection
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to create test database");
    
    // Enable foreign key constraints (disabled by default in SQLite)
    sqlx::query::<Sqlite>("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await
        .expect("Failed to enable foreign keys");
    
    // Run migrations to create schema
    sqlx::migrate!("./migrations/sqlite")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    pool
}

// ===== TEST APP STATE SETUP =====
// Creates a test application state with in-memory database
pub async fn setup_test_app_state() -> Arc<AppState> {
    // Create test database
    let sqlite_pool = setup_test_db().await;
    
    // Build test configuration
    let config = AppConfig {
        env: "test".to_string(),
        jwt_secret: "test_secret_key_for_testing_only".to_string(),
        cors_origins: "http://localhost:3000".to_string(),
        use_postgres: false,
        pg_database_url: "".to_string(),
        sqlite_database_url: "sqlite::memory:".to_string(),
    };
    
    // Create app state
    Arc::new(AppState {
        db: Arc::new(DbPool::Sqlite(sqlite_pool)),
        config: Arc::new(config),
    })
}

// Helper functions to access AppState fields
pub fn get_sqlite_pool(state: &Arc<AppState>) -> &SqlitePool {
    match state.db.as_ref() {
        DbPool::Sqlite(pool) => pool,
        _ => panic!("Expected SQLite pool"),
    }
}

pub fn get_jwt_secret(state: &Arc<AppState>) -> String {
    state.config.jwt_secret.clone()
}

// ===== TEST DATA FACTORIES =====
// Factory functions to create test data with sensible defaults

// Create a test company
pub async fn create_test_company(pool: &SqlitePool) -> Uuid {
    let company_id = Uuid::new_v4();
    
    sqlx::query::<Sqlite>(
        r#"
        INSERT INTO companies (id, name, code, email, country, is_active)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#
    )
    .bind(company_id)
    .bind("Test Company Ltd")
    .bind("TEST01")
    .bind("test@company.com")
    .bind("USA")
    .bind(true)
    .execute(pool)
    .await
    .expect("Failed to create test company");
    
    company_id
}

// Create a test role
pub async fn create_test_role(
    pool: &SqlitePool,
    company_id: Uuid,
    role_type: UserRole,
) -> Uuid {
    let role_id = Uuid::new_v4();
    
    // Full admin permissions
    let permissions = serde_json::json!({
        "users": {"create": true, "read": true, "update": true, "delete": true},
        "inventory": {"create": true, "read": true, "update": true, "delete": true},
        "sales": {"create": true, "read": true, "update": true, "delete": true, "approve": true},
        "purchases": {"create": true, "read": true, "update": true, "delete": true},
        "reports": {"view": true, "export": true}
    });
    
    sqlx::query::<Sqlite>(
        r#"
        INSERT INTO roles (
            id, company_id, name, description, role_type,
            permissions, is_system, is_active
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#
    )
    .bind(role_id)
    .bind(company_id)
    .bind("Test Admin Role")
    .bind("Full access for testing")
    .bind(role_type)
    .bind(permissions)
    .bind(true)
    .bind(true)
    .execute(pool)
    .await
    .expect("Failed to create test role");
    
    role_id
}

// Create a test user
pub async fn create_test_user(
    pool: &SqlitePool,
    company_id: Uuid,
    role_id: Uuid,
    username: &str,
) -> Uuid {
    let user_id = Uuid::new_v4();
    
    // Hash a test password
    let password_hash = crate::utils::password::hash_password("TestPass123!")
        .expect("Failed to hash password");
    
    sqlx::query::<Sqlite>(
        r#"
        INSERT INTO users (
            id, company_id, role_id, username, email, password_hash,
            first_name, last_name, status, is_email_verified,
            failed_login_attempts, two_factor_enabled, preferences
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#
    )
    .bind(user_id)
    .bind(company_id)
    .bind(role_id)
    .bind(username)
    .bind(format!("{}@test.com", username))
    .bind(password_hash)
    .bind("Test")
    .bind("User")
    .bind(UserStatus::Active)
    .bind(true)
    .bind(0)
    .bind(false)
    .bind(serde_json::json!({}))
    .execute(pool)
    .await
    .expect("Failed to create test user");
    
    user_id
}

// Create a test branch
pub async fn create_test_branch(
    pool: &SqlitePool,
    company_id: Uuid,
    code: &str,
) -> Uuid {
    let branch_id = Uuid::new_v4();
    
    sqlx::query::<Sqlite>(
        r#"
        INSERT INTO branches (
            id, company_id, code, name, type, address,
            city, state, country, postal_code, is_active
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#
    )
    .bind(branch_id)
    .bind(company_id)
    .bind(code)
    .bind(format!("Test Branch {}", code))
    .bind("warehouse")
    .bind("123 Test Street")
    .bind("Test City")
    .bind("Test State")
    .bind("USA")
    .bind("12345")
    .bind(true)
    .execute(pool)
    .await
    .expect("Failed to create test branch");
    
    branch_id
}

// Create a test inventory item
pub async fn create_test_item(
    pool: &SqlitePool,
    company_id: Uuid,
    sku: &str,
) -> Uuid {
    let item_id = Uuid::new_v4();
    
    sqlx::query::<Sqlite>(
        r#"
        INSERT INTO inventory_items (
            id, company_id, sku, name, description, unit_of_measure,
            is_serialized, is_batch_tracked, cost_price, selling_price,
            tax_rate, reorder_level, reorder_quantity, lead_time_days,
            images, specifications, tags, is_active, is_discontinued
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
        "#
    )
    .bind(item_id)
    .bind(company_id)
    .bind(sku)
    .bind(format!("Test Item {}", sku))
    .bind("Test item description")
    .bind("PCS")
    .bind(false)
    .bind(false)
    .bind("100")
    .bind("200")
    .bind("8.5")
    .bind(10)
    .bind(50)
    .bind(7)
    .bind("[]")
    .bind("{}")
    .bind("test")
    .bind(true)
    .bind(false)
    .execute(pool)
    .await
    .expect("Failed to create test item");
    
    item_id
}

// Create test stock
pub async fn create_test_stock(
    pool: &SqlitePool,
    company_id: Uuid,
    item_id: Uuid,
    branch_id: Uuid,
    quantity: i32,
) {
    let stock_id = Uuid::new_v4();
    
    sqlx::query::<Sqlite>(
        r#"
        INSERT INTO stock (
            id, company_id, item_id, branch_id, quantity_on_hand,
            quantity_allocated, quantity_in_transit
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#
    )
    .bind(stock_id)
    .bind(company_id)
    .bind(item_id)
    .bind(branch_id)
    .bind(quantity)
    .bind(0)
    .bind(0)
    .execute(pool)
    .await
    .expect("Failed to create test stock");
}

// Create a test customer
pub async fn create_test_customer(
    pool: &SqlitePool,
    company_id: Uuid,
    code: &str,
) -> Uuid {
    let customer_id = Uuid::new_v4();
    
    sqlx::query::<Sqlite>(
        r#"
        INSERT INTO customers (
            id, company_id, customer_code, name, email, phone,
            billing_address, billing_city, billing_country,
            credit_limit, credit_days, discount_percentage, is_active
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#
    )
    .bind(customer_id)
    .bind(company_id)
    .bind(code)
    .bind(format!("Test Customer {}", code))
    .bind(format!("customer{}@test.com", code))
    .bind("+1-555-0000")
    .bind("123 Customer St")
    .bind("Test City")
    .bind("USA")
    .bind("10000")
    .bind(30)
    .bind("0")
    .bind(true)
    .execute(pool)
    .await
    .expect("Failed to create test customer");
    
    customer_id
}

// Create a test supplier
pub async fn create_test_supplier(
    pool: &SqlitePool,
    company_id: Uuid,
    code: &str,
) -> Uuid {
    let supplier_id = Uuid::new_v4();
    
    sqlx::query::<Sqlite>(
        r#"
        INSERT INTO suppliers (
            id, company_id, supplier_code, name, email, phone,
            address, city, country, payment_terms, lead_time_days, is_active
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        "#
    )
    .bind(supplier_id)
    .bind(company_id)
    .bind(code)
    .bind(format!("Test Supplier {}", code))
    .bind(format!("supplier{}@test.com", code))
    .bind("+1-555-1111")
    .bind("456 Supplier Ave")
    .bind("Supplier City")
    .bind("USA")
    .bind(30)
    .bind(7)
    .bind(true)
    .execute(pool)
    .await
    .expect("Failed to create test supplier");
    
    supplier_id
}

// Generate a test JWT token
pub fn generate_test_token(
    user_id: Uuid,
    company_id: Uuid,
    role_id: Uuid,
    secret: &str,
) -> String {
    generate_jwt(
        user_id,
        company_id,
        role_id,
        "test@test.com",
        "testuser",
        TokenType::Access,
        secret,
    )
    .expect("Failed to generate test token")
}

// ===== ASSERTION HELPERS =====

// Assert that a result is successful
#[macro_export]
macro_rules! assert_ok {
    ($result:expr) => {
        assert!($result.is_ok(), "Expected Ok, got Err: {:?}", $result.err());
    };
}

// Assert that a result is an error
#[macro_export]
macro_rules! assert_err {
    ($result:expr) => {
        assert!($result.is_err(), "Expected Err, got Ok: {:?}", $result.ok());
    };
}

// Clean up test data (called after each test)
pub async fn cleanup_test_db(pool: &SqlitePool) {
    // SQLite in-memory databases are automatically cleaned up when connection closes
    // This function is here for consistency and future PostgreSQL test support
    let _ = pool.close().await;
}