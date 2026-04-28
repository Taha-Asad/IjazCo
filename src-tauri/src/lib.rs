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
        // Authentication (with auth layer for protected routes)
        .nest("/api/v1/auth", handlers::auth::auth_router().layer(auth_middleware_fn.clone()))
        // Inventory
        .nest("/api/v1/inventory", handlers::inventory::inventory_router())
        // Protected routes - add auth layer
        .nest("/api/v1/users", handlers::users::users_router().layer(auth_middleware_fn))
        // Categories
        .nest("/api/v1/categories", handlers::categories::categories_router())
        // Customers
        .nest("/api/v1/customers", handlers::customers::customers_router())
        // Sales
        .nest("/api/v1/sales", handlers::sales::sales_router())
        // Purchases
        .nest("/api/v1/purchases", handlers::purchases::purchases_router())
        // Imports
        .nest("/api/v1/imports", handlers::imports::imports_router())
        // Stock
        .nest("/api/v1/stock", handlers::stock::stock_router())
        // Suppliers
        .nest("/api/v1/suppliers", handlers::suppliers::suppliers_router())

        .nest("/api/v1/companies", handlers::companies::companies_router())
        // Inject DbPool into extensions
        .layer(db_injection)
        // Add AppState as Extension
        .layer(Extension(app_state.clone()))
        // Add AppState as State
        .with_state(state)
}