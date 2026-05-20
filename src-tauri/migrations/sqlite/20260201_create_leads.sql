-- Create leads table for SQLite
CREATE TABLE IF NOT EXISTS leads (
    id BLOB NOT NULL PRIMARY KEY,
    company_id BLOB NOT NULL,
    lead_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    company_name TEXT,
    status TEXT DEFAULT 'new',
    source TEXT DEFAULT 'other',
    estimated_value REAL,
    description TEXT,
    assigned_to BLOB,
    converted_to_customer BLOB,
    expected_close_date TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),
    created_by BLOB,
    updated_by BLOB,
    FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE,
    FOREIGN KEY (assigned_to) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (converted_to_customer) REFERENCES customers(id) ON DELETE SET NULL,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (updated_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_leads_company_id ON leads(company_id);
CREATE INDEX IF NOT EXISTS idx_leads_status ON leads(status);
CREATE INDEX IF NOT EXISTS idx_leads_assigned_to ON leads(assigned_to);
