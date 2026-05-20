-- 1. Create a Primary Company
INSERT INTO companies (
    id, name, code, email, country, currency, timezone, is_active, created_at, updated_at
) VALUES (
    X'00000000000000000000000000000001', 
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
    X'00000000000000000000000000000001', 
    X'00000000000000000000000000000001', 
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
    X'00000000000000000000000000000001', 
    X'00000000000000000000000000000001', 
    'WH-MAIN', 
    'Main Warehouse', 
    '123 Logistics Way', 
    'San Francisco', 
    'USA', 
    1, 
    datetime('now'), 
    datetime('now')
);
