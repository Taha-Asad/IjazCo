// src/lib.rs
pub mod config;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod utils;

use axum::{
    Router, 
    routing::get,
    Extension,
    middleware::from_fn, 
    extract::Request, 
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::config::{AppState, DbPool};
use crate::middleware::auth::auth_middleware;

pub fn create_router(state: Arc<AppState>) -> Router {
    // Clone db pool for injection into request extensions
    let db_pool = state.db.clone();
    let app_state = state.clone();
    
    // Middleware to inject DbPool into request extensions
    let db_injection = from_fn(move |mut req: Request, next: Next| {
        req.extensions_mut().insert(db_pool.clone());
        async move { next.run(req).await }
    });
    
    // Create auth middleware
    let auth_middleware_fn = from_fn(auth_middleware);
    
    Router::new()
        // Health check
        .nest("/api/v1/health", handlers::health::health_router())
        // Dashboard
        .nest("/api/v1/dashboard", handlers::dashboard::dashboard_router().layer(auth_middleware_fn.clone()))
        // Authentication (with auth layer for protected routes)
        .nest("/api/v1/auth", handlers::auth::auth_router().layer(auth_middleware_fn.clone()))
        // Protected routes - add auth layer
        .nest("/api/v1/users", handlers::users::users_router().layer(auth_middleware_fn.clone()))
        // Categories
        .nest("/api/v1/categories", handlers::categories::categories_router().layer(auth_middleware_fn.clone()))
        // Customers
        .nest("/api/v1/customers", handlers::customers::customers_router().layer(auth_middleware_fn.clone()))
        // Sales
        .nest("/api/v1/sales", handlers::sales::sales_router().layer(auth_middleware_fn.clone()))
        // Purchases
        .nest("/api/v1/purchases", handlers::purchases::purchases_router().layer(auth_middleware_fn.clone()))
        // Inventory
        .nest("/api/v1/inventory", handlers::inventory::inventory_router().layer(auth_middleware_fn.clone()))
        // Stock
        .nest("/api/v1/stock", handlers::stock::stock_router().layer(auth_middleware_fn.clone()))
        // Suppliers
        .nest("/api/v1/suppliers", handlers::suppliers::suppliers_router().layer(auth_middleware_fn.clone()))
        // Imports
        .nest("/api/v1/imports", handlers::imports::imports_router().layer(auth_middleware_fn.clone()))
        // Reports
        .nest("/api/v1/reports", handlers::reports::reports_router().layer(auth_middleware_fn.clone()))
        // Roles
        .nest("/api/v1/roles", handlers::roles::roles_router().layer(auth_middleware_fn.clone()))
        // Companies
        .nest("/api/v1/companies", handlers::companies::companies_router().layer(auth_middleware_fn))
        // Inject DbPool into extensions
        .layer(db_injection)
        // Add AppState as Extension
        .layer(Extension(app_state.clone()))
        // Add AppState as State
        .with_state(state)
}