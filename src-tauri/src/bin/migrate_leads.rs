use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "postgresql://neondb_owner:npg_JBFZ6dEYTM4W@ep-soft-bread-a4insfne-pooler.us-east-1.aws.neon.tech/neondb?sslmode=require&channel_binding=require";
    
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(10))
        .connect(database_url)
        .await?;
    
    println!("Connected to database!");
    
    // Create enums using DO block for compatibility
    let _ = sqlx::query(r#"
        DO $$ 
        BEGIN 
            IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'lead_status') THEN
                CREATE TYPE lead_status AS ENUM ('new', 'contacted', 'qualified', 'proposal', 'negotiation', 'won', 'lost');
            END IF;
            IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'lead_source') THEN
                CREATE TYPE lead_source AS ENUM ('website', 'referral', 'coldcall', 'socialmedia', 'email', 'other');
            END IF;
        END $$;
    "#)
    .execute(&pool)
    .await;
    println!("Created enums (if not exist)");
    
    // Create table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS leads (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
            lead_number VARCHAR(50) NOT NULL UNIQUE,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(255),
            phone VARCHAR(50),
            company_name VARCHAR(255),
            status lead_status DEFAULT 'new',
            source lead_source DEFAULT 'other',
            estimated_value DECIMAL(15, 2),
            description TEXT,
            assigned_to UUID REFERENCES users(id) ON DELETE SET NULL,
            converted_to_customer UUID REFERENCES customers(id) ON DELETE SET NULL,
            expected_close_date DATE,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW(),
            created_by UUID REFERENCES users(id) ON DELETE SET NULL,
            updated_by UUID REFERENCES users(id) ON DELETE SET NULL
        )
    "#)
    .execute(&pool)
    .await?;
    println!("Created leads table");
    
    // Create indexes
    let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_leads_company_id ON leads(company_id)").execute(&pool).await;
    let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_leads_status ON leads(status)").execute(&pool).await;
    let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_leads_assigned_to ON leads(assigned_to)").execute(&pool).await;
    
    println!("Migration completed successfully!");
    Ok(())
}
