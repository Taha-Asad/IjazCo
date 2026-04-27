-- 1. Create a Primary Company
INSERT INTO companies (
    id, name, code, email, country, currency, timezone, is_active, created_at, updated_at
) VALUES (
    '00000000-0000-0000-0000-000000000001', 
    'Acme Scientific Equipment', 
    'ACME01', 
    'admin@acme-scientific.com', 
    'USA', 
    'USD', 
    'UTC', 
    1, 
    datetime('now'), 
    datetime('now')
);

-- 2. Create the System Admin Role
INSERT INTO roles (
    id, company_id, name, role_type, permissions, is_system, is_active, created_at, updated_at
) VALUES (
    '00000000-0000-0000-0000-000000000001', 
    '00000000-0000-0000-0000-000000000001', 
    'Administrator', 
    'admin', 
    '{"users": {"create": true, "read": true, "update": true, "delete": true}, "inventory": {"create": true, "read": true, "update": true, "delete": true}}', 
    1, 
    1, 
    datetime('now'), 
    datetime('now')
);

-- 3. Create a Primary Branch
INSERT INTO branches (
    id, company_id, code, name, address, city, country, is_active, created_at, updated_at
) VALUES (
    '00000000-0000-0000-0000-000000000001', 
    '00000000-0000-0000-0000-000000000001', 
    'WH-MAIN', 
    'Main Warehouse', 
    '123 Logistics Way', 
    'San Francisco', 
    'USA', 
    1, 
    datetime('now'), 
    datetime('now')
);

-- 4. Create an Admin User (Password is 'Admin@123!')
-- Note: The hash below is a placeholder. You should register via API to get a real Argon2 hash, 
-- but this allows the database to pass Foreign Key checks.
INSERT INTO users (
    id, company_id, role_id, username, email, password_hash, first_name, last_name, status, is_email_verified, created_at, updated_at
) VALUES (
    '00000000-0000-0000-0000-000000000001', 
    '00000000-0000-0000-0000-000000000001', 
    '00000000-0000-0000-0000-000000000001', 
    'system_admin', 
    'admin@acme-scientific.com', 
    '$argon2id$v=19$m=65536,t=3,p=4$6iS86Tj6U5uI1O6Q$S4vTzJ...', 
    'System', 
    'Admin', 
    'active', 
    1, 
    datetime('now'), 
    datetime('now')
);