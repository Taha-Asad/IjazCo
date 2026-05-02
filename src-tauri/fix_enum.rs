use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("PG_DATABASE_URL")
        .expect("PG_DATABASE_URL must be set");
    
    let pool = PgPool::connect(&database_url).await?;
    
    // Check current enum values
    let rows = sqlx::query("SELECT enumlabel FROM pg_enum e JOIN pg_type t ON e.enumtypid = t.oid WHERE t.typname = 'user_role' ORDER BY enumsortorder")
        .fetch_all(&pool)
        .await?;
    
    println!("Current enum values:");
    for row in &rows {
        let label: String = row.get("enumlabel");
        println!("  - {}", label);
    }
    
    // Add missing values
    let expected = vec!["admin", "inventory_manager", "sales_user", "purchase_manager", "accountant", "import_clerk", "reports_viewer", "read_only"];
    let current: Vec<String> = rows.iter().map(|r| r.get("enumlabel")).collect();
    
    for val in expected {
        if !current.contains(&val.to_string()) {
            println!("Adding: {}", val);
            sqlx::query(&format!("ALTER TYPE user_role ADD VALUE '{}'", val))
                .execute(&pool)
                .await?;
        }
    }
    
    println!("Done!");
    Ok(())
}
