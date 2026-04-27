// src/models/mod.rs
// Central module for all data models
// Organizes entities for the ERP system

pub mod user;           
pub mod company;        
pub mod branch;         
pub mod category;       
pub mod inventory;      
pub mod stock;          
pub mod customer;       
pub mod supplier;       
pub mod sales;          
pub mod purchase;       
pub mod import;         
pub mod audit;          

// ===== RE-EXPORT COMMONLY USED TYPES =====

// User Management
pub use user::{
    User, UserSafe, UserRole, UserStatus, 
    CreateUserRequest, UpdateUserRequest, ChangePasswordRequest,
    Role, CreateRoleRequest, UpdateRoleRequest,
};

// Company & Branch
pub use company::{Company, CreateCompanyRequest, UpdateCompanyRequest};
pub use branch::{Branch, CreateBranchRequest, UpdateBranchRequest};

// Inventory & Stock
pub use category::{Category, CreateCategoryRequest, UpdateCategoryRequest};
pub use inventory::{
    InventoryItem, InventoryItemWithStock,
    CreateItemRequest, UpdateItemRequest,
};
pub use stock::{
    Stock, StockWithItem, StockMovement, StockMovementWithDetails,
    MovementType, LowStockAlert,
    UpsertStockRequest, StockAdjustmentRequest, StockTransferRequest,
};

// Sales & Customers
pub use customer::{
    Customer, CustomerWithStats,
    CreateCustomerRequest, UpdateCustomerRequest,
};
pub use sales::{
    SalesInvoice, SalesInvoiceItem, SalesInvoiceWithItems,
    InvoiceStatus, CreateSalesInvoiceRequest, RecordPaymentRequest,
};

// Purchase & Import
pub use supplier::{
    Supplier, CreateSupplierRequest, UpdateSupplierRequest,
};
pub use purchase::{
    PurchaseOrder, PurchaseOrderItem, PurchaseOrderWithItems,
    PurchaseStatus, CreatePurchaseOrderRequest, ReceiveGoodsRequest,
};
pub use import::{
    ImportOrder, ImportOrderWithDetails,
    CreateImportOrderRequest, UpdateImportOrderRequest,
};

// Audit
pub use audit::{AuditLog, AuditAction, CreateAuditLogRequest};