// src/lib.rs
pub mod config;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod utils;

use axum::{
    Router,
    middleware::from_fn,
    extract::Request,
    middleware::Next,
};
use std::sync::Arc;
use crate::config::AppState;
use crate::middleware::auth::auth_middleware;

pub fn create_router(state: Arc<AppState>) -> Router {
    // Clone state for the extension layer
    let state_for_ext = state.clone();

    // This layer adds AppState to request extensions so middleware can access it
    let add_state_ext = from_fn(move |mut req: Request, next: Next| {
        req.extensions_mut().insert(state_for_ext.clone());
        next.run(req)
    });

    let auth_middleware_fn = from_fn(auth_middleware);

    Router::new()
        // Health check (no auth required)
        .nest("/api/v1/health", handlers::health::health_router())
        // Dashboard
        .nest("/api/v1/dashboard", handlers::dashboard::dashboard_router().layer(auth_middleware_fn.clone()))
        // Authentication
        .nest("/api/v1/auth", handlers::auth::auth_router().layer(auth_middleware_fn.clone()))
        // Protected routes
        .nest("/api/v1/users", handlers::users::users_router().layer(auth_middleware_fn.clone()))
        .nest("/api/v1/categories", handlers::categories::categories_router().layer(auth_middleware_fn.clone()))
        .nest("/api/v1/customers", handlers::customers::customers_router().layer(auth_middleware_fn.clone()))
        .nest("/api/v1/sales", handlers::sales::sales_router().layer(auth_middleware_fn.clone()))
        .nest("/api/v1/purchases", handlers::purchases::purchases_router().layer(auth_middleware_fn.clone()))
        .nest("/api/v1/inventory", handlers::inventory::inventory_router().layer(auth_middleware_fn.clone()))
        .nest("/api/v1/stock", handlers::stock::stock_router().layer(auth_middleware_fn.clone()))
        .nest("/api/v1/suppliers", handlers::suppliers::suppliers_router().layer(auth_middleware_fn.clone()))
        .nest("/api/v1/imports", handlers::imports::imports_router().layer(auth_middleware_fn.clone()))
        .nest("/api/v1/reports", handlers::reports::reports_router().layer(auth_middleware_fn.clone()))
        .nest("/api/v1/roles", handlers::roles::roles_router().layer(auth_middleware_fn.clone()))
        .nest("/api/v1/companies", handlers::companies::companies_router().layer(auth_middleware_fn.clone()))
        .nest("/api/v1/leads", handlers::leads::leads_router().layer(auth_middleware_fn))
        // Add state to extensions BEFORE auth middleware runs
        .layer(add_state_ext)
        // Attach state for handlers using State extractor
        .with_state(state)
}
