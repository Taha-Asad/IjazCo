# Complete Function Reference

This document lists all functions defined in the src-tauri codebase.

---

## Configuration (`config.rs`)

| Function | Signature | Purpose |
|----------|----------|---------|
| `AppState::new()` | `pub async fn new(config: AppConfig) -> Self` | Initialize app state with DB connection |
| `AppState::pg()` | `pub fn pg(&self) -> Option<&PgPool>` | Get PostgreSQL pool |
| `AppState::sqlite()` | `pub fn sqlite(&self) -> Option<&SqlitePool>` | Get SQLite pool |
| `get_jwt_secret()` | `fn get_jwt_secret(state: &AppState) -> &str` | Extract JWT secret from state |
| `get_db_pool()` | `fn get_db_pool(state: &AppState) -> &DbPool` | Get database pool from state |

---

## Main Entry Point (`main.rs`)

| Function | Signature | Purpose |
|----------|----------|---------|
| `main()` | `#[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>>` | Application entry point |
| `shutdown_signal()` | `async fn shutdown_signal()` | Handle graceful shutdown (Ctrl+C) |

---

## Library/Router Setup (`lib.rs`)

| Function | Signature | Purpose |
|----------|----------|---------|
| `create_router()` | `pub fn create_router(state: Arc<AppState>) -> Router` | Create Axum router with all endpoints |

---

## Authentication Middleware (`middleware/auth.rs`)

| Function | Signature | Purpose |
|----------|----------|---------|
| `auth_middleware()` | `pub async fn auth_middleware(request: Request, next: Next) -> Result<Response>` | JWT authentication middleware |
| `optional_auth_middleware()` | `pub async fn optional_auth_middleware(request: Request, next: Next) -> Result<Response>` | Optional auth middleware |
| `get_auth_user()` | `pub fn get_auth_user(request: &Request) -> Result<&AuthUser>` | Extract AuthUser from request |
| `verify_company_access()` | `pub fn verify_company_access(auth_user: &AuthUser, resource_company_id: Uuid) -> Result<()>` | Multi-tenant access check |

---

## JWT Utilities (`utils/jwt.rs`)

| Function | Signature | Purpose |
|----------|----------|---------|
| `generate_jwt()` | `pub fn generate_jwt(user_id: Uuid, company_id: Uuid, role_id: Uuid, email: &str, username: &str, token_type: TokenType, secret: &str) -> Result<String>` | Generate JWT token |
| `validate_jwt()` | `pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims>` | Validate and decode JWT |
| `extract_token_from_header()` | `pub fn extract_token_from_header(auth_header: &str) -> Result<&str>` | Extract token from Authorization header |
| `refresh_access_token()` | `pub fn refresh_access_token(refresh_token: &str, secret: &str) -> Result<(String, Claims)>` | Refresh access token using refresh token |
| `TokenType::expiration_duration()` | `fn expiration_duration(&self) -> Duration` | Get token expiration duration |
| `TokenType::as_str()` | `fn as_str(&self) -> &str` | Convert token type to string |

---

## Password Utilities (`utils/password.rs`)

| Function | Signature | Purpose |
|----------|----------|---------|
| `hash_password()` | `pub fn hash_password(password: &str) -> Result<String>` | Hash password with Argon2 |
| `verify_password()` | `pub fn verify_password(password: &str, hash: &str) -> Result<bool>` | Verify password against hash |
| `validate_password_strength()` | `pub fn validate_password_strength(password: &str) -> Result<()>` | Validate password strength |
| `generate_random_password()` | `pub fn generate_random_password(length: usize) -> String` | Generate secure random password |

---

## Response Utilities (`utils/response.rs`)

| Function | Signature | Purpose |
|----------|----------|---------|
| `success()` | `pub fn success<T>(message: &str, data: T) -> ApiResponse<T>` | Create success response |
| `success_with_status()` | `pub fn success_with_status<T>(status: StatusCode, message: &str, data: T) -> impl IntoResponse` | Success with custom status |
| `paginated()` | `pub fn paginated<T>(data: Vec<T>, current_page: i64, per_page: i64, total_items: i64) -> ApiResponse<T>` | Create paginated response |
| `created()` | `pub fn created<T>(message: &str, data: T) -> impl IntoResponse` | Create "201 Created" response |
| `no_content()` | `pub fn no_content() -> impl IntoResponse` | Create "204 No Content" response |

---

## Error Handling (`utils/error.rs`)

| Type/Impl | Details |
|----------|---------|
| `AppError` enum | DatabaseError, NotFound, DuplicateKey, InvalidCredentials, InvalidToken, MissingToken, TokenExpired, AccountLocked, AccountInactive, Forbidden, ValidationError, BadRequest, InsufficientStock, InvalidStatus, CreditLimitExceeded, OperationNotAllowed, InternalError, etc. |
| `Result<T>` type | `std::result::Result<T, AppError>` |
| `IntoResponse for AppError` | Converts errors to HTTP responses with proper status codes |
| `From<sqlx::Error> for AppError` | Convert SQLx errors |
| `From<ValidationErrors> for AppError` | Convert validation errors |

---

## RBAC Middleware (`middleware/rbac.rs`)

| Function | Signature | Purpose |
|----------|----------|---------|
| `require_admin()` | `pub async fn require_admin(State(state): State<Arc<AppState>>, request: Request, next: Next) -> Result<Response>` | Require admin role |
| `require_role()` | `pub fn require_role(required_role: UserRole) -> impl Fn(...)` | Require specific role (TODO) |
| `require_permission()` | `pub fn require_permission(resource: &str, action: &str) -> impl Fn(...)` | Require permission (TODO) |
| `check_ownership()` | `pub fn check_ownership(auth_user_id: Uuid, resource_user_id: Uuid) -> Result<()>` | Check resource ownership |

---

## Dashboard Helpers (`handlers/dashboard.rs`)

| Function | Signature | Purpose |
|----------|----------|---------|
| `calculate_percentage_change()` | `fn calculate_percentage_change(current: Decimal, previous: Decimal) -> Decimal` | Calculate percentage change for metrics |

---

## User Model Functions (`models/user.rs`)

### PostgreSQL Variants
- `User::create_pg()` - Create user
- `User::find_by_id_pg()` - Find by ID
- `User::find_by_username_pg()` - Find by username
- `User::find_by_email_pg()` - Find by email
- `User::list_by_company_pg()` - List by company
- `User::update_pg()` - Update user
- `User::update_password_pg()` - Update password
- `User::delete_pg()` - Soft delete
- `User::increment_failed_login_pg()` - Track failed logins
- `User::update_last_login_pg()` - Update last login
- `User::set_reset_token_pg()` - Set password reset token
- `User::clear_reset_token_pg()` - Clear reset token

### SQLite Variants
- `User::create_sqlite()` - Create user
- `User::find_by_id_sqlite()` - Find by ID
- `User::find_by_username_sqlite()` - Find by username
- `User::find_by_email_sqlite()` - Find by email
- `User::list_by_company_sqlite()` - List by company
- `User::update_sqlite()` - Update user
- `User::update_password_sqlite()` - Update password
- `User::delete_sqlite()` - Soft delete
- `User::increment_failed_login_sqlite()` - Track failed logins
- `User::update_last_login_sqlite()` - Update last login
- `User::set_reset_token_sqlite()` - Set password reset token
- `User::clear_reset_token_sqlite()` - Clear reset token
- `User::find_by_reset_token_sqlite()` - Find by reset token
- `User::verify_email_sqlite()` - Verify email

### Validation
- `validate_password_strength()` - Validate password strength (in user.rs)

---

## Inventory Model Functions (`models/inventory.rs`)

### PostgreSQL Variants
- `InventoryItem::create_pg()` - Create inventory item
- `InventoryItem::find_by_id_pg()` - Find by ID
- `InventoryItem::find_by_sku_pg()` - Find by SKU
- `InventoryItem::list_by_company_pg()` - List by company
- `InventoryItem::list_by_category_pg()` - List by category
- `InventoryItem::search_pg()` - Search items
- `InventoryItem::update_pg()` - Update item
- `InventoryItem::delete_pg()` - Delete item
- `InventoryItem::get_with_stock_pg()` - Get with stock info
- `InventoryItem::get_low_stock_items_pg()` - Get low stock items

### SQLite Variants
- `InventoryItem::create_sqlite()` - Create inventory item
- `InventoryItem::find_by_id_sqlite()` - Find by ID
- `InventoryItem::find_by_sku_sqlite()` - Find by SKU
- `InventoryItem::list_by_company_sqlite()` - List by company
- `InventoryItem::list_by_category_sqlite()` - List by category
- `InventoryItem::search_sqlite()` - Search items
- `InventoryItem::update_sqlite()` - Update item
- `InventoryItem::delete_sqlite()` - Delete item
- `InventoryItem::get_with_stock_sqlite()` - Get with stock info
- `InventoryItem::get_low_stock_items_sqlite()` - Get low stock items

---

## Sales Model Functions (`models/sales.rs`)

### PostgreSQL Variants
- `SalesInvoice::create_pg()` - Create sales invoice
- `SalesInvoice::find_by_id_pg()` - Find by ID
- `SalesInvoice::list_by_company_pg()` - List by company
- `SalesInvoice::get_with_items_pg()` - Get with items
- `SalesInvoice::update_pg()` - Update invoice
- `SalesInvoice::delete_pg()` - Delete invoice
- `SalesInvoice::approve_pg()` - Approve invoice
- `SalesInvoice::record_payment_pg()` - Record payment
- `SalesInvoice::get_sales_summary_pg()` - Get sales summary

### SQLite Variants
- `SalesInvoice::create_sqlite()` - Create sales invoice
- `SalesInvoice::find_by_id_sqlite()` - Find by ID
- `SalesInvoice::list_by_company_sqlite()` - List by company
- `SalesInvoice::get_with_items_sqlite()` - Get with items
- `SalesInvoice::update_sqlite()` - Update invoice
- `SalesInvoice::delete_sqlite()` - Delete invoice
- `SalesInvoice::approve_sqlite()` - Approve invoice
- `SalesInvoice::record_payment_sqlite()` - Record payment
- `SalesInvoice::get_sales_summary_sqlite()` - Get sales summary

---

## Stock Model Functions (`models/stock.rs`)

### Stock Functions

#### PostgreSQL Variants
- `Stock::list_by_company_pg()` - List by company
- `Stock::list_by_branch_pg()` - List by branch
- `Stock::list_by_item_pg()` - List by item
- `Stock::adjust_quantity_pg()` - Adjust stock quantity
- `Stock::transfer_pg()` - Transfer between branches
- `Stock::record_count_pg()` - Record physical count
- `Stock::get_low_stock_items_pg()` - Get low stock items

#### SQLite Variants
- `Stock::list_by_company_sqlite()` - List by company
- `Stock::list_by_branch_sqlite()` - List by branch
- `Stock::list_by_item_sqlite()` - List by item
- `Stock::adjust_quantity_sqlite()` - Adjust stock quantity
- `Stock::transfer_sqlite()` - Transfer between branches
- `Stock::record_count_sqlite()` - Record physical count
- `Stock::get_low_stock_items_sqlite()` - Get low stock items

### StockMovement Functions

#### PostgreSQL Variants
- `StockMovement::create_pg()` - Create movement record
- `StockMovement::list_by_item_pg()` - List by item
- `StockMovement::list_by_branch_pg()` - List by branch
- `StockMovement::list_by_company_pg()` - List by company

#### SQLite Variants
- `StockMovement::create_sqlite()` - Create movement record
- `StockMovement::list_by_item_sqlite()` - List by item
- `StockMovement::list_by_branch_sqlite()` - List by branch
- `StockMovement::list_by_company_sqlite()` - List by company

---

## Company Model Functions (`models/company.rs`)

- `Company::create_pg()` / `create_sqlite()` - Create company
- `Company::find_by_id_pg()` / `find_by_id_sqlite()` - Find by ID
- `Company::find_by_slug_pg()` / `find_by_slug_sqlite()` - Find by slug
- `Company::list_all_pg()` / `list_all_sqlite()` - List all companies
- `Company::update_pg()` / `update_sqlite()` - Update company
- `Company::delete_pg()` / `delete_sqlite()` - Delete company

---

## Category Model Functions (`models/category.rs`)

- `Category::create_pg()` / `create_sqlite()` - Create category
- `Category::find_by_id_pg()` / `find_by_id_sqlite()` - Find by ID
- `Category::find_by_company_pg()` / `find_by_company_sqlite()` - Find by company
- `Category::list_by_parent_pg()` / `list_by_parent_sqlite()` - List by parent
- `Category::update_pg()` / `update_sqlite()` - Update category
- `Category::delete_pg()` / `delete_sqlite()` - Delete category

---

## Customer Model Functions (`models/customer.rs`)

- `Customer::create_pg()` / `create_sqlite()` - Create customer
- `Customer::find_by_id_pg()` / `find_by_id_sqlite()` - Find by ID
- `Customer::list_by_company_pg()` / `list_by_company_sqlite()` - List by company
- `Customer::update_pg()` / `update_sqlite()` - Update customer
- `Customer::delete_pg()` / `delete_sqlite()` - Delete customer

---

## Supplier Model Functions (`models/supplier.rs`)

- `Supplier::create_pg()` / `create_sqlite()` - Create supplier
- `Supplier::find_by_id_pg()` / `find_by_id_sqlite()` - Find by ID
- `Supplier::list_by_company_pg()` / `list_by_company_sqlite()` - List by company
- `Supplier::update_pg()` / `update_sqlite()` - Update supplier
- `Supplier::delete_pg()` / `delete_sqlite()` - Delete supplier

---

## Purchase Model Functions (`models/purchase.rs`)

- `PurchaseOrder::create_pg()` / `create_sqlite()` - Create purchase order
- `PurchaseOrder::find_by_id_pg()` / `find_by_id_sqlite()` - Find by ID
- `PurchaseOrder::list_by_company_pg()` / `list_by_company_sqlite()` - List by company
- `PurchaseOrder::update_pg()` / `update_sqlite()` - Update PO
- `PurchaseOrder::delete_pg()` / `delete_sqlite()` - Delete PO
- `PurchaseOrder::submit_pg()` / `submit_sqlite()` - Submit PO
- `PurchaseOrder::receive_pg()` / `receive_sqlite()` - Receive goods

---

## Import Model Functions (`models/import.rs`)

- `ImportOrder::create_pg()` / `create_sqlite()` - Create import order
- `ImportOrder::find_by_id_pg()` / `find_by_id_sqlite()` - Find by ID
- `ImportOrder::list_by_company_pg()` / `list_by_company_sqlite()` - List by company
- `ImportOrder::update_pg()` / `update_sqlite()` - Update import
- `ImportOrder::delete_pg()` / `delete_sqlite()` - Delete import

---

## Audit Model Functions (`models/audit.rs`)

- `AuditLog::create_pg()` / `create_sqlite()` - Create audit log entry
- `AuditLog::list_by_company_pg()` / `list_by_company_sqlite()` - List by company
- `AuditLog::list_by_user_pg()` / `list_by_user_sqlite()` - List by user
- `AuditLog::list_by_resource_pg()` / `list_by_resource_sqlite()` - List by resource

---

## Handler Functions Summary

### Health Handlers (`handlers/health.rs`)
- `root()` - Welcome message
- `health_check()` - System health with DB status
- `readiness_check()` - Readiness probe
- `liveness_check()` - Liveness probe

### Auth Handlers (`handlers/auth.rs`)
- `login()` - Authenticate user
- `register()` - Register new user
- `refresh_token()` - Refresh access token
- `logout()` - Logout user
- `me()` - Get current user info
- `change_password()` - Change own password
- `verify_email()` - Verify email address
- `request_password_reset()` - Request password reset
- `reset_password()` - Reset password with token

### User Handlers (`handlers/users.rs`)
- `list_users()` - List users (paginated)
- `get_user()` - Get user by ID
- `create_user()` - Create new user
- `update_user()` - Update user
- `delete_user()` - Soft delete user
- `admin_change_password()` - Admin change user password
- `update_user_status()` - Activate/deactivate user

### Company Handlers (`handlers/companies.rs`)
- `list_companies()` - List all companies
- `get_company()` - Get company by ID
- `create_company()` - Create new company
- `update_company()` - Update company
- `delete_company()` - Delete company

### Role Handlers (`handlers/roles.rs`)
- `list_roles()` - List roles for company
- `get_role()` - Get role by ID
- `create_role()` - Create new role
- `update_role()` - Update role
- `delete_role()` - Delete role
- `get_role_permissions()` - Get role permissions
- `update_role_permissions()` - Update role permissions

### Category Handlers (`handlers/categories.rs`)
- `list_categories()` - List categories
- `get_category()` - Get category by ID
- `create_category()` - Create category
- `update_category()` - Update category
- `delete_category()` - Delete category

### Customer Handlers (`handlers/customers.rs`)
- `list_customers()` - List customers (paginated)
- `get_customer()` - Get customer with stats
- `create_customer()` - Create customer
- `update_customer()` - Update customer
- `delete_customer()` - Delete customer

### Supplier Handlers (`handlers/suppliers.rs`)
- `list_suppliers()` - List suppliers (paginated)
- `get_supplier()` - Get supplier with stats
- `create_supplier()` - Create supplier
- `update_supplier()` - Update supplier
- `delete_supplier()` - Delete supplier

### Inventory Handlers (`handlers/inventory.rs`)
- `list_items()` - List inventory items (paginated)
- `get_item()` - Get item with stock info
- `create_item()` - Create inventory item
- `update_item()` - Update item
- `delete_item()` - Delete item
- `get_item_stock()` - Get stock levels for item
- `low_stock_items()` - Get low stock alerts

### Stock Handlers (`handlers/stock.rs`)
- `list_stock()` - List stock levels (paginated)
- `adjust_stock()` - Adjust stock quantity
- `transfer_stock()` - Transfer between branches
- `list_movements()` - List stock movements
- `physical_count()` - Record physical count
- `low_stock_alerts()` - Get low stock alerts

### Sales Handlers (`handlers/sales.rs`)
- `list_invoices()` - List invoices (paginated)
- `get_invoice()` - Get invoice with items
- `create_invoice()` - Create sales invoice
- `update_invoice()` - Update invoice (draft only)
- `delete_invoice()` - Delete invoice
- `approve_invoice()` - Approve invoice, deduct stock
- `record_payment()` - Record payment
- `get_invoice_items()` - Get invoice items
- `sales_summary()` - Get sales summary

### Purchase Handlers (`handlers/purchases.rs`)
- `list_orders()` - List purchase orders
- `get_order()` - Get PO with items
- `create_order()` - Create purchase order
- `update_order()` - Update PO (draft only)
- `delete_order()` - Delete PO
- `submit_order()` - Submit PO
- `receive_goods()` - Receive goods, add stock
- `get_order_items()` - Get PO items

### Import Handlers (`handlers/imports.rs`)
- `list_imports()` - List import orders
- `get_import()` - Get import with details
- `create_import()` - Create import order
- `update_import()` - Update import
- `delete_import()` - Delete import

### Dashboard Handlers (`handlers/dashboard.rs`)
- `get_dashboard_stats()` - Comprehensive dashboard statistics
- `sales_summary()` - Sales summary with trends
- `inventory_valuation()` - Inventory value by branch
- `sales_chart_data()` - Sales chart data (daily/monthly)

### Report Handlers (`handlers/reports.rs`)
- `sales_report()` - Generate sales report
- `inventory_report()` - Generate inventory report
- `export_pdf()` - Export report to PDF
