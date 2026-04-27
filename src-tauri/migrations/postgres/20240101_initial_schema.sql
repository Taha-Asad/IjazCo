-- migrations/20240101_initial_schema.sql
-- Complete database schema for ERP system with all tables and relationships

-- ===== ENABLE EXTENSIONS =====
-- Enable UUID generation for primary keys
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Enable pgcrypto for additional encryption functions
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ===== ENUM TYPES =====
-- Define custom ENUM types for type safety and constraints

-- User roles for RBAC (Role-Based Access Control)
CREATE TYPE user_role AS ENUM (
    'admin',                -- Full system access
    'inventory_manager',    -- Inventory and stock management
    'sales_user',          -- Sales and customer management
    'import_clerk'         -- Purchase and import management
);

-- User account status
CREATE TYPE user_status AS ENUM (
    'active',              -- Can log in and use system
    'inactive',            -- Temporarily disabled
    'suspended',           -- Blocked due to violation
    'pending'              -- Awaiting approval
);

-- Invoice status tracking
CREATE TYPE invoice_status AS ENUM (
    'draft',               -- Being created
    'pending',             -- Awaiting approval
    'approved',            -- Approved, not yet paid
    'paid',                -- Payment received
    'cancelled',           -- Cancelled invoice
    'refunded'             -- Payment refunded
);

-- Purchase order status
CREATE TYPE purchase_status AS ENUM (
    'draft',               -- Being created
    'submitted',           -- Sent to supplier
    'confirmed',           -- Supplier confirmed
    'shipped',             -- Items shipped
    'received',            -- Items received
    'cancelled'            -- Order cancelled
);

-- Stock movement types for audit trail
CREATE TYPE stock_movement_type AS ENUM (
    'purchase',            -- Added via purchase order
    'sale',                -- Deducted via sale
    'transfer',            -- Moved between branches
    'adjustment',          -- Manual correction
    'return',              -- Customer return
    'damage',              -- Damaged items
    'loss'                 -- Lost items
);

-- ===== CORE TABLES =====

-- Companies table (multi-tenant support for SaaS)
-- Each company is a separate tenant with isolated data
CREATE TABLE companies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique company identifier
    name VARCHAR(255) NOT NULL,                       -- Company legal name
    trade_name VARCHAR(255),                          -- Trading name (if different)
    registration_number VARCHAR(100) UNIQUE,          -- Business registration number
    tax_id VARCHAR(100),                              -- Tax identification number
    email VARCHAR(255) NOT NULL,                      -- Primary contact email
    phone VARCHAR(50),                                -- Primary phone number
    address TEXT,                                     -- Physical address
    city VARCHAR(100),                                -- City
    state VARCHAR(100),                               -- State/Province
    country VARCHAR(100) NOT NULL DEFAULT 'USA',      -- Country
    postal_code VARCHAR(20),                          -- ZIP/Postal code
    website VARCHAR(255),                             -- Company website
    logo_url TEXT,                                    -- Company logo URL
    currency VARCHAR(3) DEFAULT 'USD',                -- Base currency (ISO 4217)
    timezone VARCHAR(50) DEFAULT 'UTC',               -- Company timezone
    is_active BOOLEAN DEFAULT true,                   -- Active subscription status
    subscription_plan VARCHAR(50) DEFAULT 'trial',    -- Subscription tier
    subscription_expires_at TIMESTAMPTZ,              -- Subscription expiry
    max_users INTEGER DEFAULT 5,                      -- Maximum allowed users
    max_branches INTEGER DEFAULT 1,                   -- Maximum allowed branches
    features JSONB DEFAULT '{}',                      -- Enabled features (JSON)
    settings JSONB DEFAULT '{}',                      -- Company-specific settings
    created_at TIMESTAMPTZ DEFAULT NOW(),             -- Record creation timestamp
    updated_at TIMESTAMPTZ DEFAULT NOW(),             -- Last update timestamp
    created_by UUID,                                  -- User who created record
    updated_by UUID,                                  -- User who last updated
    
    -- Indexes for performance
    CONSTRAINT chk_email_format CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z]{2,}$')
);

-- Index for faster company lookups
CREATE INDEX idx_companies_active ON companies(is_active) WHERE is_active = true;
CREATE INDEX idx_companies_subscription ON companies(subscription_expires_at) WHERE is_active = true;

-- Roles table (RBAC - Role-Based Access Control)
-- Defines system roles and their permissions
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique role identifier
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,  -- Belongs to company
    name VARCHAR(100) NOT NULL,                       -- Role name (e.g., "Admin")
    description TEXT,                                 -- Role description
    role_type user_role NOT NULL,                     -- Predefined role type
    permissions JSONB NOT NULL DEFAULT '{}',          -- Permission matrix (JSON)
    is_system BOOLEAN DEFAULT false,                  -- System-defined (cannot be deleted)
    is_active BOOLEAN DEFAULT true,                   -- Active status
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,
    
    -- Ensure unique role names per company
    CONSTRAINT uq_role_per_company UNIQUE(company_id, name)
);

-- Index for role lookups
CREATE INDEX idx_roles_company ON roles(company_id) WHERE is_active = true;
CREATE INDEX idx_roles_type ON roles(role_type);

-- Users table (authentication and authorization)
-- Stores user accounts with secure password hashing
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique user identifier
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,  -- User's company
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE RESTRICT,  -- User's role
    username VARCHAR(100) NOT NULL UNIQUE,            -- Unique username for login
    email VARCHAR(255) NOT NULL UNIQUE,               -- Unique email address
    password_hash VARCHAR(255) NOT NULL,              -- Argon2 hashed password
    first_name VARCHAR(100) NOT NULL,                 -- User's first name
    last_name VARCHAR(100) NOT NULL,                  -- User's last name
    phone VARCHAR(50),                                -- Contact phone
    avatar_url TEXT,                                  -- Profile picture URL
    status user_status DEFAULT 'pending',             -- Account status
    is_email_verified BOOLEAN DEFAULT false,          -- Email verification status
    email_verified_at TIMESTAMPTZ,                    -- Email verification timestamp
    last_login_at TIMESTAMPTZ,                        -- Last successful login
    last_login_ip INET,                               -- Last login IP address
    failed_login_attempts INTEGER DEFAULT 0,          -- Track failed logins (security)
    locked_until TIMESTAMPTZ,                         -- Account lock timestamp
    password_reset_token VARCHAR(255),                -- Password reset token
    password_reset_expires_at TIMESTAMPTZ,            -- Token expiration
    two_factor_enabled BOOLEAN DEFAULT false,         -- 2FA enabled
    two_factor_secret VARCHAR(255),                   -- 2FA secret key
    preferences JSONB DEFAULT '{}',                   -- User preferences (theme, etc.)
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,
    deleted_at TIMESTAMPTZ,                           -- Soft delete timestamp
    
    -- Constraints
    CONSTRAINT chk_username_length CHECK (length(username) >= 3),
    CONSTRAINT chk_email_format CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z]{2,}$')
);

-- Indexes for user lookups and authentication
CREATE INDEX idx_users_company ON users(company_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_email ON users(email) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_username ON users(username) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_status ON users(status) WHERE status = 'active';

-- Branches/Warehouses table (multi-location support)
-- Manages physical locations for inventory storage
CREATE TABLE branches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique branch identifier
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,  -- Belongs to company
    code VARCHAR(50) NOT NULL,                        -- Branch code (e.g., "WH-001")
    name VARCHAR(255) NOT NULL,                       -- Branch name
    type VARCHAR(50) DEFAULT 'warehouse',             -- Type (warehouse, showroom, office)
    email VARCHAR(255),                               -- Branch email
    phone VARCHAR(50),                                -- Branch phone
    manager_id UUID REFERENCES users(id) ON DELETE SET NULL,  -- Branch manager
    address TEXT NOT NULL,                            -- Physical address
    city VARCHAR(100) NOT NULL,                       -- City
    state VARCHAR(100),                               -- State/Province
    country VARCHAR(100) NOT NULL DEFAULT 'USA',      -- Country
    postal_code VARCHAR(20),                          -- ZIP/Postal code
    latitude DECIMAL(10, 8),                          -- GPS latitude
    longitude DECIMAL(11, 8),                         -- GPS longitude
    capacity_sqft DECIMAL(10, 2),                     -- Storage capacity (square feet)
    is_active BOOLEAN DEFAULT true,                   -- Active status
    settings JSONB DEFAULT '{}',                      -- Branch-specific settings
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,
    
    -- Ensure unique branch codes per company
    CONSTRAINT uq_branch_code_per_company UNIQUE(company_id, code)
);

-- Index for branch lookups
CREATE INDEX idx_branches_company ON branches(company_id) WHERE is_active = true;
CREATE INDEX idx_branches_manager ON branches(manager_id);

-- Categories table (hierarchical product categorization)
-- Organizes inventory items into categories
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique category identifier
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,  -- Belongs to company
    parent_id UUID REFERENCES categories(id) ON DELETE CASCADE,  -- Parent category (for hierarchy)
    code VARCHAR(50) NOT NULL,                        -- Category code (e.g., "CAT-001")
    name VARCHAR(255) NOT NULL,                       -- Category name
    description TEXT,                                 -- Category description
    image_url TEXT,                                   -- Category image
    sort_order INTEGER DEFAULT 0,                     -- Display order
    is_active BOOLEAN DEFAULT true,                   -- Active status
    metadata JSONB DEFAULT '{}',                      -- Additional attributes
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,
    
    -- Ensure unique category codes per company
    CONSTRAINT uq_category_code_per_company UNIQUE(company_id, code),
    -- Prevent self-referencing
    CONSTRAINT chk_no_self_reference CHECK (id != parent_id)
);

-- Index for category hierarchy and lookups
CREATE INDEX idx_categories_company ON categories(company_id) WHERE is_active = true;
CREATE INDEX idx_categories_parent ON categories(parent_id);

-- Inventory Items table (product master data)
-- Central repository for all inventory items
CREATE TABLE inventory_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique item identifier
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,  -- Belongs to company
    category_id UUID REFERENCES categories(id) ON DELETE SET NULL,  -- Item category
    sku VARCHAR(100) NOT NULL,                        -- Stock Keeping Unit (unique)
    barcode VARCHAR(100),                             -- Barcode/EAN/UPC
    name VARCHAR(255) NOT NULL,                       -- Item name
    description TEXT,                                 -- Detailed description
    brand VARCHAR(100),                               -- Brand/Manufacturer
    model_number VARCHAR(100),                        -- Model number
    serial_number VARCHAR(100),                       -- Serial number (for serialized items)
    unit_of_measure VARCHAR(50) DEFAULT 'PCS',        -- Unit (PCS, KG, L, etc.)
    is_serialized BOOLEAN DEFAULT false,              -- Track by serial number
    is_batch_tracked BOOLEAN DEFAULT false,           -- Track by batch/lot number
    cost_price DECIMAL(15, 2) NOT NULL DEFAULT 0,     -- Purchase cost
    selling_price DECIMAL(15, 2) NOT NULL DEFAULT 0,  -- Selling price
    msrp DECIMAL(15, 2),                              -- Manufacturer's suggested retail price
    tax_rate DECIMAL(5, 2) DEFAULT 0,                 -- Tax rate percentage
    weight DECIMAL(10, 3),                            -- Item weight
    weight_unit VARCHAR(10) DEFAULT 'KG',             -- Weight unit
    dimensions JSONB,                                 -- Dimensions (L x W x H)
    reorder_level INTEGER DEFAULT 10,                 -- Minimum stock level for reorder alert
    reorder_quantity INTEGER DEFAULT 50,              -- Suggested reorder quantity
    max_stock_level INTEGER,                          -- Maximum stock level
    lead_time_days INTEGER DEFAULT 7,                 -- Supplier lead time in days
    warranty_period INTEGER,                          -- Warranty in months
    images JSONB DEFAULT '[]',                        -- Product images array
    specifications JSONB DEFAULT '{}',                -- Technical specifications
    tags TEXT[],                                      -- Searchable tags
    is_active BOOLEAN DEFAULT true,                   -- Active for sale
    is_discontinued BOOLEAN DEFAULT false,            -- Discontinued product
    discontinued_at TIMESTAMPTZ,                      -- Discontinuation date
    metadata JSONB DEFAULT '{}',                      -- Custom fields
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,
    
    -- Ensure unique SKU per company
    CONSTRAINT uq_sku_per_company UNIQUE(company_id, sku),
    -- Price validation
    CONSTRAINT chk_positive_cost CHECK (cost_price >= 0),
    CONSTRAINT chk_positive_price CHECK (selling_price >= 0)
);

-- Indexes for inventory lookups and searches
CREATE INDEX idx_inventory_company ON inventory_items(company_id) WHERE is_active = true;
CREATE INDEX idx_inventory_category ON inventory_items(category_id);
CREATE INDEX idx_inventory_sku ON inventory_items(sku);
CREATE INDEX idx_inventory_barcode ON inventory_items(barcode) WHERE barcode IS NOT NULL;
CREATE INDEX idx_inventory_name ON inventory_items USING gin(to_tsvector('english', name));  -- Full-text search

-- Stock table (inventory levels per branch)
-- Tracks actual stock quantities at each location
CREATE TABLE stock (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique stock record identifier
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,  -- Belongs to company
    item_id UUID NOT NULL REFERENCES inventory_items(id) ON DELETE CASCADE,  -- Inventory item
    branch_id UUID NOT NULL REFERENCES branches(id) ON DELETE CASCADE,  -- Storage location
    quantity_on_hand INTEGER NOT NULL DEFAULT 0,      -- Current physical stock
    quantity_allocated INTEGER NOT NULL DEFAULT 0,    -- Reserved for orders
    quantity_available INTEGER GENERATED ALWAYS AS (quantity_on_hand - quantity_allocated) STORED,  -- Available stock
    quantity_in_transit INTEGER NOT NULL DEFAULT 0,   -- In-transit from supplier
    bin_location VARCHAR(100),                        -- Physical bin/shelf location
    last_counted_at TIMESTAMPTZ,                      -- Last physical count date
    last_count_qty INTEGER,                           -- Last counted quantity
    variance INTEGER,                                 -- Count variance
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Ensure one stock record per item per branch
    CONSTRAINT uq_stock_item_branch UNIQUE(item_id, branch_id),
    -- Stock cannot be negative
    CONSTRAINT chk_non_negative_stock CHECK (quantity_on_hand >= 0),
    CONSTRAINT chk_non_negative_allocated CHECK (quantity_allocated >= 0)
);

-- Indexes for stock lookups
CREATE INDEX idx_stock_company ON stock(company_id);
CREATE INDEX idx_stock_item ON stock(item_id);
CREATE INDEX idx_stock_branch ON stock(branch_id);
CREATE INDEX idx_stock_low ON stock(item_id) WHERE quantity_available < 10;  -- Low stock alerts

-- Stock Movements table (audit trail for all stock changes)
-- Complete history of inventory movements for traceability
CREATE TABLE stock_movements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique movement identifier
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,  -- Belongs to company
    item_id UUID NOT NULL REFERENCES inventory_items(id) ON DELETE CASCADE,  -- Affected item
    from_branch_id UUID REFERENCES branches(id) ON DELETE SET NULL,  -- Source branch
    to_branch_id UUID REFERENCES branches(id) ON DELETE SET NULL,    -- Destination branch
    movement_type stock_movement_type NOT NULL,       -- Type of movement
    quantity INTEGER NOT NULL,                        -- Quantity moved
    unit_cost DECIMAL(15, 2),                         -- Cost per unit
    reference_type VARCHAR(50),                       -- Reference document type (sale, purchase, etc.)
    reference_id UUID,                                -- Reference document ID
    batch_number VARCHAR(100),                        -- Batch/Lot number
    serial_numbers TEXT[],                            -- Serial numbers (for serialized items)
    notes TEXT,                                       -- Movement notes
    movement_date TIMESTAMPTZ DEFAULT NOW(),          -- Movement date
    created_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,  -- User responsible
    
    -- Validation
    CONSTRAINT chk_positive_quantity CHECK (quantity > 0),
    CONSTRAINT chk_branch_logic CHECK (
        CASE
            WHEN movement_type = 'transfer' THEN from_branch_id IS NOT NULL AND to_branch_id IS NOT NULL
            ELSE true
        END
    )
);

-- Indexes for movement tracking
CREATE INDEX idx_movements_company ON stock_movements(company_id);
CREATE INDEX idx_movements_item ON stock_movements(item_id);
CREATE INDEX idx_movements_date ON stock_movements(movement_date DESC);
CREATE INDEX idx_movements_reference ON stock_movements(reference_type, reference_id);

-- Customers table (sales contacts)
-- Manages customer information for sales transactions
CREATE TABLE customers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique customer identifier
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,  -- Belongs to company
    customer_code VARCHAR(50) NOT NULL,               -- Customer code (e.g., "CUST-001")
    name VARCHAR(255) NOT NULL,                       -- Customer name
    contact_person VARCHAR(255),                      -- Primary contact person
    email VARCHAR(255),                               -- Email address
    phone VARCHAR(50),                                -- Phone number
    mobile VARCHAR(50),                               -- Mobile number
    tax_id VARCHAR(100),                              -- Tax ID/VAT number
    billing_address TEXT,                             -- Billing address
    billing_city VARCHAR(100),                        -- Billing city
    billing_state VARCHAR(100),                       -- Billing state
    billing_country VARCHAR(100) DEFAULT 'USA',       -- Billing country
    billing_postal_code VARCHAR(20),                  -- Billing ZIP code
    shipping_address TEXT,                            -- Shipping address
    shipping_city VARCHAR(100),                       -- Shipping city
    shipping_state VARCHAR(100),                      -- Shipping state
    shipping_country VARCHAR(100) DEFAULT 'USA',      -- Shipping country
    shipping_postal_code VARCHAR(20),                 -- Shipping ZIP code
    credit_limit DECIMAL(15, 2) DEFAULT 0,            -- Maximum credit allowed
    credit_days INTEGER DEFAULT 30,                   -- Payment terms (days)
    discount_percentage DECIMAL(5, 2) DEFAULT 0,      -- Default discount
    is_active BOOLEAN DEFAULT true,                   -- Active customer
    tags TEXT[],                                      -- Customer tags
    notes TEXT,                                       -- Internal notes
    metadata JSONB DEFAULT '{}',                      -- Custom fields
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,
    
    -- Ensure unique customer codes per company
    CONSTRAINT uq_customer_code_per_company UNIQUE(company_id, customer_code)
);

-- Indexes for customer lookups
CREATE INDEX idx_customers_company ON customers(company_id) WHERE is_active = true;
CREATE INDEX idx_customers_code ON customers(customer_code);
CREATE INDEX idx_customers_name ON customers USING gin(to_tsvector('english', name));

-- Suppliers table (purchase contacts)
-- Manages supplier information for procurement
CREATE TABLE suppliers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique supplier identifier
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,  -- Belongs to company
    supplier_code VARCHAR(50) NOT NULL,               -- Supplier code (e.g., "SUPP-001")
    name VARCHAR(255) NOT NULL,                       -- Supplier name
    contact_person VARCHAR(255),                      -- Primary contact person
    email VARCHAR(255),                               -- Email address
    phone VARCHAR(50),                                -- Phone number
    website VARCHAR(255),                             -- Supplier website
    tax_id VARCHAR(100),                              -- Tax ID/VAT number
    address TEXT,                                     -- Physical address
    city VARCHAR(100),                                -- City
    state VARCHAR(100),                               -- State/Province
    country VARCHAR(100) DEFAULT 'USA',               -- Country
    postal_code VARCHAR(20),                          -- ZIP/Postal code
    payment_terms INTEGER DEFAULT 30,                 -- Payment terms (days)
    lead_time_days INTEGER DEFAULT 7,                 -- Standard lead time
    rating DECIMAL(3, 2),                             -- Supplier rating (0-5)
    is_active BOOLEAN DEFAULT true,                   -- Active supplier
    tags TEXT[],                                      -- Supplier tags
    notes TEXT,                                       -- Internal notes
    metadata JSONB DEFAULT '{}',                      -- Custom fields
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,
    
    -- Ensure unique supplier codes per company
    CONSTRAINT uq_supplier_code_per_company UNIQUE(company_id, supplier_code)
);

-- Indexes for supplier lookups
CREATE INDEX idx_suppliers_company ON suppliers(company_id) WHERE is_active = true;
CREATE INDEX idx_suppliers_code ON suppliers(supplier_code);

-- Sales Invoices table (sales transactions)
-- Records all sales transactions with customers
CREATE TABLE sales_invoices (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique invoice identifier
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,  -- Belongs to company
    branch_id UUID NOT NULL REFERENCES branches(id) ON DELETE RESTRICT,  -- Issuing branch
    customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE RESTRICT,  -- Customer
    invoice_number VARCHAR(100) NOT NULL,             -- Invoice number (auto-generated)
    invoice_date DATE NOT NULL DEFAULT CURRENT_DATE,  -- Invoice date
    due_date DATE,                                    -- Payment due date
    status invoice_status DEFAULT 'draft',            -- Invoice status
    subtotal DECIMAL(15, 2) NOT NULL DEFAULT 0,       -- Sum of line items
    discount_amount DECIMAL(15, 2) DEFAULT 0,         -- Total discount
    tax_amount DECIMAL(15, 2) DEFAULT 0,              -- Total tax
    shipping_amount DECIMAL(15, 2) DEFAULT 0,         -- Shipping charges
    total_amount DECIMAL(15, 2) NOT NULL DEFAULT 0,   -- Final total
    paid_amount DECIMAL(15, 2) DEFAULT 0,             -- Amount paid
    balance_due DECIMAL(15, 2) GENERATED ALWAYS AS (total_amount - paid_amount) STORED,  -- Remaining balance
    payment_method VARCHAR(50),                       -- Payment method
    payment_reference VARCHAR(255),                   -- Payment reference number
    shipping_address TEXT,                            -- Delivery address
    notes TEXT,                                       -- Invoice notes
    terms_and_conditions TEXT,                        -- Terms & conditions
    metadata JSONB DEFAULT '{}',                      -- Custom fields
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,  -- Creator
    updated_by UUID,
    
    -- Ensure unique invoice numbers per company
    CONSTRAINT uq_invoice_number_per_company UNIQUE(company_id, invoice_number),
    -- Amount validation
    CONSTRAINT chk_positive_subtotal CHECK (subtotal >= 0),
    CONSTRAINT chk_positive_total CHECK (total_amount >= 0),
    CONSTRAINT chk_paid_not_exceed CHECK (paid_amount <= total_amount)
);

-- Indexes for invoice lookups
CREATE INDEX idx_invoices_company ON sales_invoices(company_id);
CREATE INDEX idx_invoices_customer ON sales_invoices(customer_id);
CREATE INDEX idx_invoices_branch ON sales_invoices(branch_id);
CREATE INDEX idx_invoices_date ON sales_invoices(invoice_date DESC);
CREATE INDEX idx_invoices_status ON sales_invoices(status);
CREATE INDEX idx_invoices_due ON sales_invoices(due_date) WHERE status IN ('approved', 'pending');

-- Sales Invoice Items table (line items)
-- Detailed breakdown of items in each invoice
CREATE TABLE sales_invoice_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique line item identifier
    invoice_id UUID NOT NULL REFERENCES sales_invoices(id) ON DELETE CASCADE,  -- Parent invoice
    item_id UUID NOT NULL REFERENCES inventory_items(id) ON DELETE RESTRICT,  -- Sold item
    description TEXT,                                 -- Item description (override)
    quantity INTEGER NOT NULL,                        -- Quantity sold
    unit_price DECIMAL(15, 2) NOT NULL,               -- Price per unit
    discount_percentage DECIMAL(5, 2) DEFAULT 0,      -- Line discount %
    discount_amount DECIMAL(15, 2) DEFAULT 0,         -- Line discount amount
    tax_percentage DECIMAL(5, 2) DEFAULT 0,           -- Line tax %
    tax_amount DECIMAL(15, 2) DEFAULT 0,              -- Line tax amount
    line_total DECIMAL(15, 2) GENERATED ALWAYS AS (
        (quantity * unit_price) - discount_amount + tax_amount
    ) STORED,                                         -- Line total (computed)
    serial_numbers TEXT[],                            -- Serial numbers (if applicable)
    batch_number VARCHAR(100),                        -- Batch number
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Validation
    CONSTRAINT chk_positive_quantity CHECK (quantity > 0),
    CONSTRAINT chk_positive_unit_price CHECK (unit_price >= 0)
);

-- Indexes for invoice items
CREATE INDEX idx_invoice_items_invoice ON sales_invoice_items(invoice_id);
CREATE INDEX idx_invoice_items_item ON sales_invoice_items(item_id);

-- Purchase Orders table (procurement)
-- Manages purchase orders to suppliers
CREATE TABLE purchase_orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique PO identifier
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,  -- Belongs to company
    branch_id UUID NOT NULL REFERENCES branches(id) ON DELETE RESTRICT,  -- Receiving branch
    supplier_id UUID NOT NULL REFERENCES suppliers(id) ON DELETE RESTRICT,  -- Supplier
    po_number VARCHAR(100) NOT NULL,                  -- PO number (auto-generated)
    po_date DATE NOT NULL DEFAULT CURRENT_DATE,       -- PO date
    expected_delivery_date DATE,                      -- Expected delivery
    status purchase_status DEFAULT 'draft',           -- PO status
    subtotal DECIMAL(15, 2) NOT NULL DEFAULT 0,       -- Sum of line items
    discount_amount DECIMAL(15, 2) DEFAULT 0,         -- Total discount
    tax_amount DECIMAL(15, 2) DEFAULT 0,              -- Total tax
    shipping_amount DECIMAL(15, 2) DEFAULT 0,         -- Shipping charges
    total_amount DECIMAL(15, 2) NOT NULL DEFAULT 0,   -- Final total
    currency VARCHAR(3) DEFAULT 'USD',                -- Currency
    exchange_rate DECIMAL(10, 4) DEFAULT 1,           -- Exchange rate
    payment_terms INTEGER DEFAULT 30,                 -- Payment terms (days)
    shipping_address TEXT,                            -- Delivery address
    notes TEXT,                                       -- PO notes
    metadata JSONB DEFAULT '{}',                      -- Custom fields
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,  -- Creator
    updated_by UUID,
    
    -- Ensure unique PO numbers per company
    CONSTRAINT uq_po_number_per_company UNIQUE(company_id, po_number)
);

-- Indexes for purchase orders
CREATE INDEX idx_purchase_orders_company ON purchase_orders(company_id);
CREATE INDEX idx_purchase_orders_supplier ON purchase_orders(supplier_id);
CREATE INDEX idx_purchase_orders_branch ON purchase_orders(branch_id);
CREATE INDEX idx_purchase_orders_date ON purchase_orders(po_date DESC);
CREATE INDEX idx_purchase_orders_status ON purchase_orders(status);

-- Purchase Order Items table (line items)
-- Detailed breakdown of items in each purchase order
CREATE TABLE purchase_order_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique line item identifier
    po_id UUID NOT NULL REFERENCES purchase_orders(id) ON DELETE CASCADE,  -- Parent PO
    item_id UUID NOT NULL REFERENCES inventory_items(id) ON DELETE RESTRICT,  -- Ordered item
    description TEXT,                                 -- Item description
    quantity_ordered INTEGER NOT NULL,                -- Ordered quantity
    quantity_received INTEGER DEFAULT 0,              -- Received quantity
    unit_cost DECIMAL(15, 2) NOT NULL,                -- Cost per unit
    tax_percentage DECIMAL(5, 2) DEFAULT 0,           -- Line tax %
    tax_amount DECIMAL(15, 2) DEFAULT 0,              -- Line tax amount
    line_total DECIMAL(15, 2) GENERATED ALWAYS AS (
        (quantity_ordered * unit_cost) + tax_amount
    ) STORED,                                         -- Line total (computed)
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Validation
    CONSTRAINT chk_positive_qty_ordered CHECK (quantity_ordered > 0),
    CONSTRAINT chk_received_not_exceed CHECK (quantity_received <= quantity_ordered)
);

-- Indexes for PO items
CREATE INDEX idx_po_items_po ON purchase_order_items(po_id);
CREATE INDEX idx_po_items_item ON purchase_order_items(item_id);

-- Import Orders table (international shipments)
-- Tracks import shipments with customs and duties
CREATE TABLE import_orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique import order identifier
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,  -- Belongs to company
    po_id UUID REFERENCES purchase_orders(id) ON DELETE SET NULL,  -- Related PO
    import_number VARCHAR(100) NOT NULL,              -- Import reference number
    supplier_id UUID NOT NULL REFERENCES suppliers(id) ON DELETE RESTRICT,  -- Supplier
    shipment_date DATE,                               -- Shipment date
    arrival_date DATE,                                -- Expected/actual arrival
    customs_clearance_date DATE,                      -- Customs clearance date
    status VARCHAR(50) DEFAULT 'in_transit',          -- Shipment status
    shipping_method VARCHAR(100),                     -- Shipping method (air, sea)
    tracking_number VARCHAR(255),                     -- Tracking number
    container_number VARCHAR(100),                    -- Container number
    freight_cost DECIMAL(15, 2) DEFAULT 0,            -- Freight charges
    insurance_cost DECIMAL(15, 2) DEFAULT 0,          -- Insurance cost
    customs_duty DECIMAL(15, 2) DEFAULT 0,            -- Customs duty
    other_charges DECIMAL(15, 2) DEFAULT 0,           -- Miscellaneous charges
    total_cost DECIMAL(15, 2) GENERATED ALWAYS AS (
        freight_cost + insurance_cost + customs_duty + other_charges
    ) STORED,                                         -- Total import cost
    documents JSONB DEFAULT '[]',                     -- Import documents (invoices, BOL, etc.)
    notes TEXT,                                       -- Import notes
    metadata JSONB DEFAULT '{}',                      -- Custom fields
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    updated_by UUID,
    
    -- Ensure unique import numbers per company
    CONSTRAINT uq_import_number_per_company UNIQUE(company_id, import_number)
);

-- Indexes for import orders
CREATE INDEX idx_import_orders_company ON import_orders(company_id);
CREATE INDEX idx_import_orders_po ON import_orders(po_id);
CREATE INDEX idx_import_orders_supplier ON import_orders(supplier_id);
CREATE INDEX idx_import_orders_status ON import_orders(status);

-- Audit Log table (complete system audit trail)
-- Tracks all critical actions for compliance and security
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique log entry identifier
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,  -- Belongs to company
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,  -- User who performed action
    action VARCHAR(100) NOT NULL,                     -- Action type (CREATE, UPDATE, DELETE, LOGIN)
    entity_type VARCHAR(100) NOT NULL,                -- Affected entity (user, invoice, stock)
    entity_id UUID,                                   -- Affected entity ID
    old_values JSONB,                                 -- Previous values (for updates)
    new_values JSONB,                                 -- New values
    ip_address INET,                                  -- User's IP address
    user_agent TEXT,                                  -- User's browser/client
    metadata JSONB DEFAULT '{}',                      -- Additional context
    created_at TIMESTAMPTZ DEFAULT NOW(),             -- Timestamp
    
    -- Index for audit queries
    CHECK (action IN ('CREATE', 'UPDATE', 'DELETE', 'LOGIN', 'LOGOUT', 'EXPORT', 'IMPORT'))
);

-- Indexes for audit log searches
CREATE INDEX idx_audit_company ON audit_logs(company_id);
CREATE INDEX idx_audit_user ON audit_logs(user_id);
CREATE INDEX idx_audit_entity ON audit_logs(entity_type, entity_id);
CREATE INDEX idx_audit_created ON audit_logs(created_at DESC);
CREATE INDEX idx_audit_action ON audit_logs(action);

-- ===== TRIGGERS =====

-- Trigger function to update updated_at timestamp
-- Automatically sets updated_at to current time on any UPDATE
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();  -- Set updated_at to current timestamp
    RETURN NEW;              -- Return the modified row
END;
$$ LANGUAGE plpgsql;

-- Apply updated_at trigger to all relevant tables
CREATE TRIGGER update_companies_updated_at BEFORE UPDATE ON companies
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_roles_updated_at BEFORE UPDATE ON roles
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_branches_updated_at BEFORE UPDATE ON branches
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_categories_updated_at BEFORE UPDATE ON categories
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_inventory_items_updated_at BEFORE UPDATE ON inventory_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_stock_updated_at BEFORE UPDATE ON stock
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_customers_updated_at BEFORE UPDATE ON customers
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_suppliers_updated_at BEFORE UPDATE ON suppliers
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_sales_invoices_updated_at BEFORE UPDATE ON sales_invoices
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_purchase_orders_updated_at BEFORE UPDATE ON purchase_orders
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_import_orders_updated_at BEFORE UPDATE ON import_orders
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Trigger function to automatically update stock on invoice creation
-- Deducts stock quantity when a sale is made
CREATE OR REPLACE FUNCTION update_stock_on_sale()
RETURNS TRIGGER AS $$
BEGIN
    -- Only process if invoice is approved
    IF NEW.status = 'approved' AND (OLD.status IS NULL OR OLD.status != 'approved') THEN
        -- Deduct stock for each line item
        UPDATE stock
        SET quantity_on_hand = quantity_on_hand - sales_invoice_items.quantity
        FROM sales_invoice_items
        WHERE stock.item_id = sales_invoice_items.item_id
          AND stock.branch_id = NEW.branch_id
          AND sales_invoice_items.invoice_id = NEW.id;
        
        -- Create stock movement records
        INSERT INTO stock_movements (
            company_id, item_id, to_branch_id, movement_type,
            quantity, unit_cost, reference_type, reference_id, created_by
        )
        SELECT
            NEW.company_id,
            item_id,
            NEW.branch_id,
            'sale',
            quantity,
            unit_price,
            'sales_invoice',
            NEW.id,
            NEW.created_by
        FROM sales_invoice_items
        WHERE invoice_id = NEW.id;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply stock update trigger to sales invoices
CREATE TRIGGER trigger_update_stock_on_sale
    AFTER INSERT OR UPDATE ON sales_invoices
    FOR EACH ROW
    EXECUTE FUNCTION update_stock_on_sale();

-- Trigger function to update stock on purchase order receipt
-- Adds stock quantity when goods are received
CREATE OR REPLACE FUNCTION update_stock_on_purchase()
RETURNS TRIGGER AS $$
BEGIN
    -- Only process if PO status is 'received'
    IF NEW.status = 'received' AND (OLD.status IS NULL OR OLD.status != 'received') THEN
        -- Add stock for each line item
        INSERT INTO stock (company_id, item_id, branch_id, quantity_on_hand)
        SELECT
            NEW.company_id,
            item_id,
            NEW.branch_id,
            quantity_received
        FROM purchase_order_items
        WHERE po_id = NEW.id
        ON CONFLICT (item_id, branch_id)
        DO UPDATE SET quantity_on_hand = stock.quantity_on_hand + EXCLUDED.quantity_on_hand;
        
        -- Create stock movement records
        INSERT INTO stock_movements (
            company_id, item_id, to_branch_id, movement_type,
            quantity, unit_cost, reference_type, reference_id, created_by
        )
        SELECT
            NEW.company_id,
            item_id,
            NEW.branch_id,
            'purchase',
            quantity_received,
            unit_cost,
            'purchase_order',
            NEW.id,
            NEW.created_by
        FROM purchase_order_items
        WHERE po_id = NEW.id;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply stock update trigger to purchase orders
CREATE TRIGGER trigger_update_stock_on_purchase
    AFTER INSERT OR UPDATE ON purchase_orders
    FOR EACH ROW
    EXECUTE FUNCTION update_stock_on_purchase();

-- Trigger function for audit logging
-- Automatically creates audit log entries for critical tables
CREATE OR REPLACE FUNCTION audit_log_changes()
RETURNS TRIGGER AS $$
BEGIN
    -- Determine action type
    IF TG_OP = 'INSERT' THEN
        INSERT INTO audit_logs (company_id, user_id, action, entity_type, entity_id, new_values)
        VALUES (
            COALESCE(NEW.company_id, '00000000-0000-0000-0000-000000000000'),
            COALESCE(NEW.created_by, '00000000-0000-0000-0000-000000000000'),
            'CREATE',
            TG_TABLE_NAME,
            NEW.id,
            to_jsonb(NEW)
        );
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO audit_logs (company_id, user_id, action, entity_type, entity_id, old_values, new_values)
        VALUES (
            COALESCE(NEW.company_id, '00000000-0000-0000-0000-000000000000'),
            COALESCE(NEW.updated_by, '00000000-0000-0000-0000-000000000000'),
            'UPDATE',
            TG_TABLE_NAME,
            NEW.id,
            to_jsonb(OLD),
            to_jsonb(NEW)
        );
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO audit_logs (company_id, user_id, action, entity_type, entity_id, old_values)
        VALUES (
            COALESCE(OLD.company_id, '00000000-0000-0000-0000-000000000000'),
            COALESCE(OLD.updated_by, '00000000-0000-0000-0000-000000000000'),
            'DELETE',
            TG_TABLE_NAME,
            OLD.id,
            to_jsonb(OLD)
        );
    END IF;
    
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Apply audit trigger to critical tables
CREATE TRIGGER audit_users AFTER INSERT OR UPDATE OR DELETE ON users
    FOR EACH ROW EXECUTE FUNCTION audit_log_changes();

CREATE TRIGGER audit_sales_invoices AFTER INSERT OR UPDATE OR DELETE ON sales_invoices
    FOR EACH ROW EXECUTE FUNCTION audit_log_changes();

CREATE TRIGGER audit_purchase_orders AFTER INSERT OR UPDATE OR DELETE ON purchase_orders
    FOR EACH ROW EXECUTE FUNCTION audit_log_changes();

CREATE TRIGGER audit_stock_movements AFTER INSERT ON stock_movements
    FOR EACH ROW EXECUTE FUNCTION audit_log_changes();

-- ===== VIEWS =====

-- View for low stock items (alerts)
-- Shows items below reorder level for proactive restocking
CREATE VIEW v_low_stock_items AS
SELECT
    i.id AS item_id,
    i.company_id,
    i.sku,
    i.name,
    i.reorder_level,
    i.reorder_quantity,
    b.id AS branch_id,
    b.name AS branch_name,
    s.quantity_available,
    s.quantity_on_hand,
    s.quantity_allocated,
    (i.reorder_level - s.quantity_available) AS shortage_quantity
FROM inventory_items i
JOIN stock s ON i.id = s.item_id
JOIN branches b ON s.branch_id = b.id
WHERE s.quantity_available < i.reorder_level
  AND i.is_active = true
  AND b.is_active = true;

-- View for sales summary
-- Aggregated sales data for reporting
CREATE VIEW v_sales_summary AS
SELECT
    si.company_id,
    si.branch_id,
    b.name AS branch_name,
    c.name AS customer_name,
    DATE_TRUNC('month', si.invoice_date) AS month,
    COUNT(si.id) AS total_invoices,
    SUM(si.total_amount) AS total_sales,
    SUM(si.paid_amount) AS total_paid,
    SUM(si.balance_due) AS total_outstanding
FROM sales_invoices si
JOIN branches b ON si.branch_id = b.id
JOIN customers c ON si.customer_id = c.id
WHERE si.status != 'cancelled'
GROUP BY si.company_id, si.branch_id, b.name, c.name, DATE_TRUNC('month', si.invoice_date);

-- View for inventory valuation
-- Calculate total inventory value at cost
CREATE VIEW v_inventory_valuation AS
SELECT
    i.company_id,
    b.id AS branch_id,
    b.name AS branch_name,
    SUM(s.quantity_on_hand * i.cost_price) AS total_cost_value,
    SUM(s.quantity_on_hand * i.selling_price) AS total_selling_value,
    SUM(s.quantity_on_hand * i.selling_price) - SUM(s.quantity_on_hand * i.cost_price) AS potential_profit
FROM inventory_items i
JOIN stock s ON i.id = s.item_id
JOIN branches b ON s.branch_id = b.id
WHERE i.is_active = true
GROUP BY i.company_id, b.id, b.name;

-- ===== COMMENTS (Database Documentation) =====
COMMENT ON TABLE companies IS 'Multi-tenant companies for SaaS model';
COMMENT ON TABLE users IS 'System users with RBAC authentication';
COMMENT ON TABLE roles IS 'Role definitions with permission matrix';
COMMENT ON TABLE branches IS 'Physical locations/warehouses';
COMMENT ON TABLE inventory_items IS 'Master product catalog';
COMMENT ON TABLE stock IS 'Real-time inventory levels per branch';
COMMENT ON TABLE stock_movements IS 'Complete audit trail of stock changes';
COMMENT ON TABLE sales_invoices IS 'Customer sales transactions';
COMMENT ON TABLE purchase_orders IS 'Supplier purchase orders';
COMMENT ON TABLE import_orders IS 'International import shipments';
COMMENT ON TABLE audit_logs IS 'System-wide audit trail for compliance';

-- Database setup complete!
-- Run this migration to create the complete schema
ALTER TABLE suppliers 
ADD COLUMN rating NUMERIC(3,2) CHECK (rating >= 0 AND rating <= 5);