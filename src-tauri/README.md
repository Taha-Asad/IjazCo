# Tauri ERP API Documentation

## Overview

This is a Rust/Tauri 2.0 ERP backend with Axum web framework. It supports dual database (PostgreSQL for cloud, SQLite for desktop) and provides comprehensive REST APIs for inventory management, sales, purchases, and user administration.

**Base URL:** `/api/v1`

---

## Table of Contents

1. [Health Check](#health-check)
2. [Authentication](#authentication)
3. [User Management](#user-management)
4. [Company Management](#company-management)
5. [Role & Permission Management](#role--permission-management)
6. [Category Management](#category-management)
7. [Customer Management](#customer-management)
8. [Supplier Management](#supplier-management)
9. [Inventory Management](#inventory-management)
10. [Stock Management](#stock-management)
11. [Sales Invoices](#sales-invoices)
12. [Purchase Orders](#purchase-orders)
13. [Import Orders](#import-orders)
14. [Dashboard](#dashboard)
15. [Reports](#reports)

---

## Health Check

**Router:** `health_router()`
**Base Path:** `/api/v1/health`

| Method | Endpoint | Function | Description |
|--------|----------|----------|-------------|
| GET | `/` | `root()` | Welcome message |
| GET | `/health` | `health_check()` | System health with DB status |
| GET | `/ready` | `readiness_check()` | Readiness probe |
| GET | `/live` | `liveness_check()` | Liveness probe |

---

## Authentication

**Router:** `auth_router()`
**Base Path:** `/api/v1/auth`

| Method | Endpoint | Function | Description |
|--------|----------|----------|-------------|
| POST | `/login` | `login()` | Authenticate user, return JWT tokens |
| POST | `/register` | `register()` | Register new user |
| POST | `/refresh` | `refresh_token()` | Refresh access token |
| POST | `/logout` | `logout()` | Logout (client-side token discard) |
| GET | `/me` | `me()` | Get current user info |
| POST | `/change-password` | `change_password()` | Change own password |
| POST | `/verify-email` | `verify_email()` | Verify email address |
| POST | `/request-password-reset` | `request_password_reset()` | Request password reset |
| POST | `/reset-password` | `reset_password()` | Reset password with token |

---

## User Management

**Router:** `users_router()`
**Base Path:** `/api/v1/users`

| Method | Endpoint | Function | Description |
|--------|----------|----------|-------------|
| GET | `/` | `list_users()` | List users (paginated) |
| GET | `/{id}` | `get_user()` | Get user by ID |
| POST | `/` | `create_user()` | Create new user |
| PUT | `/{id}` | `update_user()` | Update user |
| DELETE | `/{id}` | `delete_user()` | Soft delete user |
| POST | `/{id}/change-password` | `admin_change_password()` | Admin change user password |
| PATCH | `/{id}/status` | `update_user_status()` | Activate/deactivate user |

---

## Company Management

**Router:** `companies_router()`
**Base Path:** `/api/v1/companies`

| Method | Endpoint | Function | Description |
|--------|----------|----------|-------------|
| GET | `/` | `list_companies()` | List all companies |
| GET | `/{id}` | `get_company()` | Get company by ID |
| POST | `/` | `create_company()` | Create new company |
| PUT | `/{id}` | `update_company()` | Update company |
| DELETE | `/{id}` | `delete_company()` | Delete company |

---

## Role & Permission Management

**Router:** `roles_router()`
**Base Path:** `/api/v1/roles`

| Method | Endpoint | Function | Description |
|--------|----------|----------|-------------|
| GET | `/` | `list_roles()` | List roles for company |
| GET | `/{id}` | `get_role()` | Get role by ID |
| POST | `/` | `create_role()` | Create new role |
| PUT | `/{id}` | `update_role()` | Update role |
| DELETE | `/{id}` | `delete_role()` | Delete role |
| GET | `/{id}/permissions` | `get_role_permissions()` | Get role permissions |
| PUT | `/{id}/permissions` | `update_role_permissions()` | Update role permissions |

---

## Category Management

**Router:** `categories_router()`
**Base Path:** `/api/v1/categories`

| Method | Endpoint | Function | Description |
|--------|----------|----------|-------------|
| GET | `/` | `list_categories()` | List categories |
| GET | `/{id}` | `get_category()` | Get category by ID |
| POST | `/` | `create_category()` | Create category |
| PUT | `/{id}` | `update_category()` | Update category |
| DELETE | `/{id}` | `delete_category()` | Delete category |

---

## Customer Management

**Router:** `customers_router()`
**Base Path:** `/api/v1/customers`

| Method | Endpoint | Function | Description |
|--------|----------|----------|-------------|
| GET | `/` | `list_customers()` | List customers (paginated) |
| GET | `/{id}` | `get_customer()` | Get customer with stats |
| POST | `/` | `create_customer()` | Create customer |
| PUT | `/{id}` | `update_customer()` | Update customer |
| DELETE | `/{id}` | `delete_customer()` | Delete customer |

---

## Supplier Management

**Router:** `suppliers_router()`
**Base Path:** `/api/v1/suppliers`

| Method | Endpoint | Function | Description |
|--------|----------|----------|-------------|
| GET | `/` | `list_suppliers()` | List suppliers (paginated) |
| GET | `/{id}` | `get_supplier()` | Get supplier with stats |
| POST | `/` | `create_supplier()` | Create supplier |
| PUT | `/{id}` | `update_supplier()` | Update supplier |
| DELETE | `/{id}` | `delete_supplier()` | Delete supplier |

---

## Inventory Management

**Router:** `inventory_router()`
**Base Path:** `/api/v1/inventory`

| Method | Endpoint | Function | Description |
|--------|----------|----------|-------------|
| GET | `/` | `list_items()` | List inventory items (paginated) |
| GET | `/{id}` | `get_item()` | Get item with stock info |
| POST | `/` | `create_item()` | Create inventory item |
| PUT | `/{id}` | `update_item()` | Update item |
| DELETE | `/{id}` | `delete_item()` | Delete item |
| GET | `/{id}/stock` | `get_item_stock()` | Get stock levels for item |
| GET | `/low-stock` | `low_stock_items()` | Get low stock alerts |

---

## Stock Management

**Router:** `stock_router()`
**Base Path:** `/api/v1/stock`

| Method | Endpoint | Function | Description |
|--------|----------|----------|-------------|
| GET | `/` | `list_stock()` | List stock levels (paginated) |
| POST | `/adjust` | `adjust_stock()` | Adjust stock quantity |
| POST | `/transfer` | `transfer_stock()` | Transfer between branches |
| GET | `/movements` | `list_movements()` | List stock movements |
| POST | `/physical-count` | `physical_count()` | Record physical count |
| GET | `/low-stock-alerts` | `low_stock_alerts()` | Get low stock alerts |

---

## Sales Invoices

**Router:** `sales_router()`
**Base Path:** `/api/v1/sales/invoices`

| Method | Endpoint | Function | Description |
|--------|----------|----------|-------------|
| GET | `/` | `list_invoices()` | List invoices (paginated) |
| GET | `/{id}` | `get_invoice()` | Get invoice with items |
| POST | `/` | `create_invoice()` | Create sales invoice |
| PUT | `/{id}` | `update_invoice()` | Update invoice (draft only) |
| DELETE | `/{id}` | `delete_invoice()` | Delete invoice |
| POST | `/{id}/approve` | `approve_invoice()` | Approve invoice, deduct stock |
| POST | `/{id}/payment` | `record_payment()` | Record payment |
| GET | `/{id}/items` | `get_invoice_items()` | Get invoice items |
| GET | `/summary` | `sales_summary()` | Get sales summary |

---

## Purchase Orders

**Router:** `purchases_router()`
**Base Path:** `/api/v1/purchases/orders`

| Method | Endpoint | Function | Description |
|--------|----------|----------|-------------|
| GET | `/` | `list_orders()` | List purchase orders |
| GET | `/{id}` | `get_order()` | Get PO with items |
| POST | `/` | `create_order()` | Create purchase order |
| PUT | `/{id}` | `update_order()` | Update PO (draft only) |
| DELETE | `/{id}` | `delete_order()` | Delete PO |
| POST | `/{id}/submit` | `submit_order()` | Submit PO |
| POST | `/{id}/receive` | `receive_goods()` | Receive goods, add stock |
| GET | `/{id}/items` | `get_order_items()` | Get PO items |

---

## Import Orders

**Router:** `imports_router()`
**Base Path:** `/api/v1/imports`

| Method | Endpoint | Function | Description |
|--------|----------|----------|-------------|
| GET | `/` | `list_imports()` | List import orders |
| GET | `/{id}` | `get_import()` | Get import with details |
| POST | `/` | `create_import()` | Create import order |
| PUT | `/{id}` | `update_import()` | Update import |
| DELETE | `/{id}` | `delete_import()` | Delete import |

---

## Dashboard

**Router:** `dashboard_router()`
**Base Path:** `/api/v1/dashboard`

| Method | Endpoint | Function | Description |
|--------|----------|----------|-------------|
| GET | `/stats` | `get_dashboard_stats()` | Comprehensive dashboard statistics |
| GET | `/sales-summary` | `sales_summary()` | Sales summary with trends |
| GET | `/inventory-valuation` | `inventory_valuation()` | Inventory value by branch |
| GET | `/sales-chart` | `sales_chart_data()` | Sales chart data (daily/monthly) |

---

## Reports

**Router:** `reports_router()`
**Base Path:** `/api/v1/reports`

| Method | Endpoint | Function | Description |
|--------|----------|----------|-------------|
| GET | `/sales` | `sales_report()` | Generate sales report |
| GET | `/inventory` | `inventory_report()` | Generate inventory report |
| POST | `/export/pdf` | `export_pdf()` | Export report to PDF |

---

## Authentication & Security

- **JWT Authentication:** Stateless auth with access/refresh token pattern
- **Password Security:** Argon2 hashing with strength validation
- **Multi-Tenancy:** Company-based data isolation with `verify_company_access()`
- **Role-Based Access Control (RBAC):** Roles with JSON permissions
- **Rate Limiting:** Configurable rate limiting middleware
- **CORS:** Configurable CORS handling

---

## Database Schema

Key tables: companies, roles, users, branches, categories, customers, suppliers, inventory_items, stock, sales_invoices, sales_invoice_items, purchase_orders, purchase_order_items, import_orders, stock_movements, audit_logs
