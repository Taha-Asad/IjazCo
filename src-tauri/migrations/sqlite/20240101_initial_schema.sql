-- SQLite-compatible initial schema
-- Complete database schema for ERP system

-- ===== COMPANIES TABLE =====
CREATE TABLE companies (
    id BLOB NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    trade_name TEXT,
    registration_number TEXT UNIQUE,
    tax_id TEXT,
    email TEXT NOT NULL,
    phone TEXT,
    address TEXT,
    city TEXT,
    state TEXT,
    country TEXT NOT NULL DEFAULT 'Pakistan',
    postal_code TEXT,
    website TEXT,
    logo_url TEXT,
    currency TEXT DEFAULT 'PKR',
    timezone TEXT DEFAULT 'UTC',
    is_active INTEGER NOT NULL DEFAULT 1,
    subscription_plan TEXT DEFAULT 'trial',
    subscription_expires_at TEXT,
    max_users INTEGER DEFAULT 5,
    max_branches INTEGER DEFAULT 1,
    features TEXT NOT NULL DEFAULT '{}',
    settings TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by BLOB,
    updated_by BLOB
);

CREATE INDEX idx_companies_active ON companies(is_active);

-- ===== ROLES TABLE =====
CREATE TABLE roles (
    id BLOB NOT NULL PRIMARY KEY,
    company_id BLOB NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    role_type TEXT NOT NULL CHECK(role_type IN ('admin', 'inventory_manager', 'sales_user', 'import_clerk')),
    permissions TEXT NOT NULL DEFAULT '{}',
    is_system INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by BLOB,
    updated_by BLOB,
    UNIQUE(company_id, name)
);

CREATE INDEX idx_roles_company ON roles(company_id);
CREATE INDEX idx_roles_type ON roles(role_type);

-- ===== USERS TABLE =====
CREATE TABLE users (
    id BLOB NOT NULL PRIMARY KEY,
    company_id BLOB NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    role_id BLOB NOT NULL REFERENCES roles(id) ON DELETE RESTRICT,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    phone TEXT,
    avatar_url TEXT,
    status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('active', 'inactive', 'suspended', 'pending')),
    is_email_verified INTEGER NOT NULL DEFAULT 0,
    email_verified_at TEXT,
    last_login_at TEXT,
    last_login_ip TEXT,
    failed_login_attempts INTEGER NOT NULL DEFAULT 0,
    locked_until TEXT,
    password_reset_token TEXT,
    password_reset_expires_at TEXT,
    two_factor_enabled INTEGER NOT NULL DEFAULT 0,
    two_factor_secret TEXT,
    preferences TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by BLOB,
    updated_by BLOB,
    deleted_at TEXT
);

CREATE INDEX idx_users_company ON users(company_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);

-- ===== BRANCHES TABLE =====
CREATE TABLE branches (
    id BLOB NOT NULL PRIMARY KEY,
    company_id BLOB NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    type TEXT DEFAULT 'warehouse',
    email TEXT,
    phone TEXT,
    manager_id BLOB REFERENCES users(id) ON DELETE SET NULL,
    address TEXT NOT NULL,
    city TEXT NOT NULL,
    state TEXT,
    country TEXT NOT NULL DEFAULT 'USA',
    postal_code TEXT,
    latitude REAL,
    longitude REAL,
    capacity_sqft REAL,
    is_active INTEGER NOT NULL DEFAULT 1,
    settings TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by BLOB,
    updated_by BLOB,
    UNIQUE(company_id, code)
);

CREATE INDEX idx_branches_company ON branches(company_id);

-- ===== CATEGORIES TABLE =====
CREATE TABLE categories (
    id BLOB NOT NULL PRIMARY KEY,
    company_id BLOB NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    parent_id BLOB REFERENCES categories(id) ON DELETE CASCADE,
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    image_url TEXT,
    sort_order INTEGER DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    metadata TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by BLOB,
    updated_by BLOB,
    UNIQUE(company_id, code)
);

CREATE INDEX idx_categories_company ON categories(company_id);
CREATE INDEX idx_categories_parent ON categories(parent_id);

-- ===== INVENTORY ITEMS TABLE =====
CREATE TABLE inventory_items (
    id BLOB NOT NULL PRIMARY KEY,
    company_id BLOB NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    category_id BLOB REFERENCES categories(id) ON DELETE SET NULL,
    sku TEXT NOT NULL,
    barcode TEXT,
    name TEXT NOT NULL,
    description TEXT,
    brand TEXT,
    model_number TEXT,
    serial_number TEXT,
    unit_of_measure TEXT DEFAULT 'PCS',
    is_serialized INTEGER NOT NULL DEFAULT 0,
    is_batch_tracked INTEGER NOT NULL DEFAULT 0,
    cost_price REAL NOT NULL DEFAULT 0,
    selling_price REAL NOT NULL DEFAULT 0,
    msrp REAL,
    tax_rate REAL DEFAULT 0,
    weight REAL,
    weight_unit TEXT DEFAULT 'KG',
    dimensions TEXT,
    reorder_level INTEGER DEFAULT 10,
    reorder_quantity INTEGER DEFAULT 50,
    max_stock_level INTEGER,
    lead_time_days INTEGER DEFAULT 7,
    warranty_period INTEGER,
    images TEXT NOT NULL DEFAULT '[]',
    specifications TEXT NOT NULL DEFAULT '{}',
    tags TEXT NOT NULL DEFAULT '[]',
    is_active INTEGER NOT NULL DEFAULT 1,
    is_discontinued INTEGER NOT NULL DEFAULT 0,
    discontinued_at TEXT,
    metadata TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by BLOB,
    updated_by BLOB,
    UNIQUE(company_id, sku)
);

CREATE INDEX idx_inventory_company ON inventory_items(company_id);
CREATE INDEX idx_inventory_category ON inventory_items(category_id);
CREATE INDEX idx_inventory_sku ON inventory_items(sku);

-- ===== STOCK TABLE =====
CREATE TABLE stock (
    id BLOB NOT NULL PRIMARY KEY,
    company_id BLOB NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    item_id BLOB NOT NULL REFERENCES inventory_items(id) ON DELETE CASCADE,
    branch_id BLOB NOT NULL REFERENCES branches(id) ON DELETE CASCADE,
    quantity_on_hand INTEGER NOT NULL DEFAULT 0,
    quantity_allocated INTEGER NOT NULL DEFAULT 0,
    quantity_available INTEGER NOT NULL DEFAULT 0,
    quantity_in_transit INTEGER NOT NULL DEFAULT 0,
    bin_location TEXT,
    last_counted_at TEXT,
    last_count_qty INTEGER,
    variance INTEGER,
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(item_id, branch_id)
);

CREATE INDEX idx_stock_company ON stock(company_id);
CREATE INDEX idx_stock_item ON stock(item_id);
CREATE INDEX idx_stock_branch ON stock(branch_id);

-- ===== STOCK MOVEMENTS TABLE =====
CREATE TABLE stock_movements (
    id BLOB NOT NULL PRIMARY KEY,
    company_id BLOB NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    item_id BLOB NOT NULL REFERENCES inventory_items(id) ON DELETE CASCADE,
    from_branch_id BLOB REFERENCES branches(id) ON DELETE SET NULL,
    to_branch_id BLOB REFERENCES branches(id) ON DELETE SET NULL,
    movement_type TEXT NOT NULL CHECK(movement_type IN ('purchase', 'sale', 'transfer', 'adjustment', 'return', 'damage', 'loss')),
    quantity INTEGER NOT NULL,
    unit_cost REAL,
    reference_type TEXT,
    reference_id TEXT,
    batch_number TEXT,
    serial_numbers TEXT NOT NULL DEFAULT '[]',
    notes TEXT,
    movement_date TEXT NOT NULL DEFAULT (datetime('now')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by BLOB NOT NULL REFERENCES users(id) ON DELETE RESTRICT
);

CREATE INDEX idx_movements_company ON stock_movements(company_id);
CREATE INDEX idx_movements_item ON stock_movements(item_id);
CREATE INDEX idx_movements_date ON stock_movements(movement_date);

-- ===== CUSTOMERS TABLE =====
CREATE TABLE customers (
    id BLOB NOT NULL PRIMARY KEY,
    company_id BLOB NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    customer_code TEXT NOT NULL,
    name TEXT NOT NULL,
    contact_person TEXT,
    email TEXT,
    phone TEXT,
    mobile TEXT,
    tax_id TEXT,
    billing_address TEXT,
    billing_city TEXT,
    billing_state TEXT,
    billing_country TEXT DEFAULT 'USA',
    billing_postal_code TEXT,
    shipping_address TEXT,
    shipping_city TEXT,
    shipping_state TEXT,
    shipping_country TEXT DEFAULT 'USA',
    shipping_postal_code TEXT,
    credit_limit REAL DEFAULT 0,
    credit_days INTEGER DEFAULT 30,
    discount_percentage REAL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    tags TEXT NOT NULL DEFAULT '[]',
    notes TEXT,
    metadata TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by BLOB,
    updated_by BLOB,
    UNIQUE(company_id, customer_code)
);

CREATE INDEX idx_customers_company ON customers(company_id);

-- ===== SUPPLIERS TABLE =====
CREATE TABLE suppliers (
    id BLOB NOT NULL PRIMARY KEY,
    company_id BLOB NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    supplier_code TEXT NOT NULL,
    name TEXT NOT NULL,
    contact_person TEXT,
    email TEXT,
    phone TEXT,
    website TEXT,
    tax_id TEXT,
    address TEXT,
    city TEXT,
    state TEXT,
    country TEXT DEFAULT 'USA',
    postal_code TEXT,
    payment_terms INTEGER DEFAULT 30,
    lead_time_days INTEGER DEFAULT 7,
    rating REAL,
    is_active INTEGER NOT NULL DEFAULT 1,
    tags TEXT NOT NULL DEFAULT '[]',
    notes TEXT,
    metadata TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by BLOB,
    updated_by BLOB,
    UNIQUE(company_id, supplier_code)
);

CREATE INDEX idx_suppliers_company ON suppliers(company_id);

-- ===== SALES INVOICES TABLE =====
CREATE TABLE sales_invoices (
    id BLOB NOT NULL PRIMARY KEY,
    company_id BLOB NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    branch_id BLOB NOT NULL REFERENCES branches(id) ON DELETE RESTRICT,
    customer_id BLOB NOT NULL REFERENCES customers(id) ON DELETE RESTRICT,
    invoice_number TEXT NOT NULL,
    invoice_date TEXT NOT NULL DEFAULT (date('now')),
    due_date TEXT,
    status TEXT NOT NULL DEFAULT 'draft' CHECK(status IN ('draft', 'pending', 'approved', 'paid', 'cancelled', 'refunded')),
    subtotal REAL NOT NULL DEFAULT 0,
    discount_amount REAL DEFAULT 0,
    tax_amount REAL DEFAULT 0,
    shipping_amount REAL DEFAULT 0,
    total_amount REAL NOT NULL DEFAULT 0,
    paid_amount REAL DEFAULT 0,
    balance_due REAL NOT NULL DEFAULT 0,
    payment_method TEXT,
    payment_reference TEXT,
    shipping_address TEXT,
    notes TEXT,
    terms_and_conditions TEXT,
    metadata TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by BLOB NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    updated_by BLOB,
    UNIQUE(company_id, invoice_number)
);

CREATE INDEX idx_invoices_company ON sales_invoices(company_id);
CREATE INDEX idx_invoices_customer ON sales_invoices(customer_id);
CREATE INDEX idx_invoices_date ON sales_invoices(invoice_date);
CREATE INDEX idx_invoices_status ON sales_invoices(status);

-- ===== SALES INVOICE ITEMS TABLE =====
CREATE TABLE sales_invoice_items (
    id BLOB NOT NULL PRIMARY KEY,
    invoice_id BLOB NOT NULL REFERENCES sales_invoices(id) ON DELETE CASCADE,
    item_id BLOB NOT NULL REFERENCES inventory_items(id) ON DELETE RESTRICT,
    description TEXT,
    quantity INTEGER NOT NULL,
    unit_price REAL NOT NULL,
    discount_percentage REAL DEFAULT 0,
    discount_amount REAL DEFAULT 0,
    tax_percentage REAL DEFAULT 0,
    tax_amount REAL DEFAULT 0,
    line_total REAL NOT NULL DEFAULT 0,
    serial_numbers TEXT NOT NULL DEFAULT '[]',
    batch_number TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_invoice_items_invoice ON sales_invoice_items(invoice_id);

-- ===== PURCHASE ORDERS TABLE =====
CREATE TABLE purchase_orders (
    id BLOB NOT NULL PRIMARY KEY,
    company_id BLOB NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    branch_id BLOB NOT NULL REFERENCES branches(id) ON DELETE RESTRICT,
    supplier_id BLOB NOT NULL REFERENCES suppliers(id) ON DELETE RESTRICT,
    po_number TEXT NOT NULL,
    po_date TEXT NOT NULL DEFAULT (date('now')),
    expected_delivery_date TEXT,
    status TEXT NOT NULL DEFAULT 'draft' CHECK(status IN ('draft', 'submitted', 'confirmed', 'shipped', 'received', 'cancelled')),
    subtotal REAL NOT NULL DEFAULT 0,
    discount_amount REAL DEFAULT 0,
    tax_amount REAL DEFAULT 0,
    shipping_amount REAL DEFAULT 0,
    total_amount REAL NOT NULL DEFAULT 0,
    currency TEXT DEFAULT 'USD',
    exchange_rate REAL DEFAULT 1,
    payment_terms INTEGER DEFAULT 30,
    shipping_address TEXT,
    notes TEXT,
    metadata TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by BLOB NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    updated_by BLOB,
    UNIQUE(company_id, po_number)
);

CREATE INDEX idx_purchase_orders_company ON purchase_orders(company_id);
CREATE INDEX idx_purchase_orders_supplier ON purchase_orders(supplier_id);
CREATE INDEX idx_purchase_orders_status ON purchase_orders(status);

-- ===== PURCHASE ORDER ITEMS TABLE =====
CREATE TABLE purchase_order_items (
    id BLOB NOT NULL PRIMARY KEY,
    po_id BLOB NOT NULL REFERENCES purchase_orders(id) ON DELETE CASCADE,
    item_id BLOB NOT NULL REFERENCES inventory_items(id) ON DELETE RESTRICT,
    description TEXT,
    quantity_ordered INTEGER NOT NULL,
    quantity_received INTEGER DEFAULT 0,
    unit_cost REAL NOT NULL,
    tax_percentage REAL DEFAULT 0,
    tax_amount REAL DEFAULT 0,
    line_total REAL NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_po_items_po ON purchase_order_items(po_id);

-- ===== IMPORT ORDERS TABLE =====
CREATE TABLE import_orders (
    id BLOB NOT NULL PRIMARY KEY,
    company_id BLOB NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    po_id BLOB REFERENCES purchase_orders(id) ON DELETE SET NULL,
    import_number TEXT NOT NULL,
    supplier_id BLOB NOT NULL REFERENCES suppliers(id) ON DELETE RESTRICT,
    shipment_date TEXT,
    arrival_date TEXT,
    customs_clearance_date TEXT,
    status TEXT DEFAULT 'in_transit',
    shipping_method TEXT,
    tracking_number TEXT,
    container_number TEXT,
    freight_cost REAL DEFAULT 0,
    insurance_cost REAL DEFAULT 0,
    customs_duty REAL DEFAULT 0,
    other_charges REAL DEFAULT 0,
    total_cost REAL NOT NULL DEFAULT 0,
    documents TEXT NOT NULL DEFAULT '[]',
    notes TEXT,
    metadata TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by BLOB NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    updated_by BLOB,
    UNIQUE(company_id, import_number)
);

CREATE INDEX idx_import_orders_company ON import_orders(company_id);

-- ===== AUDIT LOGS TABLE =====
CREATE TABLE audit_logs (
    id BLOB NOT NULL PRIMARY KEY,
    company_id BLOB NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    user_id BLOB REFERENCES users(id) ON DELETE SET NULL,
    action TEXT NOT NULL CHECK(action IN ('CREATE', 'UPDATE', 'DELETE', 'LOGIN', 'LOGOUT', 'EXPORT', 'IMPORT')),
    entity_type TEXT NOT NULL,
    entity_id TEXT,
    old_values TEXT,
    new_values TEXT,
    ip_address TEXT,
    user_agent TEXT,
    metadata TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_audit_company ON audit_logs(company_id);
CREATE INDEX idx_audit_user ON audit_logs(user_id);
CREATE INDEX idx_audit_created ON audit_logs(created_at);
