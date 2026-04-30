// src/handlers/customers.rs
// Customer management endpoints
// CRUD operations for customers with statistics

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::DbPool,
    middleware::auth::{verify_company_access, AuthUser},
    models::customer::{CreateCustomerRequest, Customer, CustomerWithStats, UpdateCustomerRequest},
    utils::{
        error::{AppError, Result},
        response::{created, no_content, paginated},
    },
    AppState,
};

use super::users::PaginationParams;

// ===== CUSTOMER SEARCH PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct CustomerSearchParams {
    pub search: Option<String>,
    
    #[serde(default = "default_true")]
    pub active_only: bool,
    
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

fn default_true() -> bool { true }

// ===== LIST CUSTOMERS ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/customers",
    tag = "customers",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("search" = Option<String>, Query, description = "Search term"),
        ("active_only" = Option<bool>, Query, description = "Show only active customers"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
    ),
    responses(
        (status = 200, description = "List of customers", body = Vec<Customer>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_customers(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<CustomerSearchParams>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        search = ?params.search,
        "Listing customers"
    );
    
    // Fetch customers based on search criteria
    let (customers, total_count) = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let customers = if let Some(ref search_term) = params.search {
                Customer::search_pg(
                    pool,
                    auth_user.company_id,
                    search_term,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else {
                Customer::list_by_company_pg(
                    pool,
                    auth_user.company_id,
                    params.active_only,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            };
            
            let total_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM customers WHERE company_id = $1 AND ($2 = false OR is_active = true)"
            )
            .bind(auth_user.company_id)
            .bind(params.active_only)
            .fetch_one(pool)
            .await?;
            
            (customers, total_count)
        }
        DbPool::Sqlite(pool) => {
            let customers = if let Some(ref search_term) = params.search {
                Customer::search_sqlite(
                    pool,
                    auth_user.company_id,
                    search_term,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else {
                Customer::list_by_company_sqlite(
                    pool,
                    auth_user.company_id,
                    params.active_only,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            };
            
            let total_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM customers WHERE company_id = ? AND (? = 0 OR is_active = 1)"
            )
            .bind(auth_user.company_id)
            .bind(params.active_only)
            .fetch_one(pool)
            .await?;
            
            (customers, total_count)
        }
    };
    
    tracing::debug!(
        count = customers.len(),
        total = total_count,
        "Customers retrieved successfully"
    );
    
    Ok(paginated(
        customers,
        params.pagination.page,
        params.pagination.per_page,
        total_count,
    ))
}

// ===== GET CUSTOMER ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/customers/{id}",
    tag = "customers",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Customer ID")
    ),
    responses(
        (status = 200, description = "Customer found", body = CustomerWithStats),
        (status = 404, description = "Customer not found"),
        (status = 403, description = "Access denied")
    )
)]
pub async fn get_customer(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<CustomerWithStats>> {
    tracing::debug!(
        user_id = %auth_user.id,
        customer_id = %id,
        "Fetching customer"
    );
    
    // Fetch customer with statistics
    let customer = match state.db.as_ref() {
        DbPool::Postgres(pool) => Customer::get_with_stats_pg(pool, id).await?,
        DbPool::Sqlite(pool) => Customer::get_with_stats_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Customer not found".to_string()))?;
    
    // Verify customer belongs to same company
    verify_company_access(&auth_user, customer.customer.company_id)?;
    
    tracing::info!(
        customer_id = %id,
        name = %customer.customer.name,
        "Customer retrieved successfully"
    );
    
    Ok(Json(customer))
}

// ===== CREATE CUSTOMER ENDPOINT =====
#[utoipa::path(
    post,
    path = "/api/v1/customers",
    tag = "customers",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateCustomerRequest,
    responses(
        (status = 201, description = "Customer created successfully", body = Customer),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Customer code already exists"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_customer(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(mut payload): Json<CreateCustomerRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        customer_code = %payload.customer_code,
        "Creating customer"
    );
    
    // Force company_id to match authenticated user's company
    payload.company_id = auth_user.company_id;
    
    // Check if customer code already exists
    let code_exists = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            Customer::find_by_code_pg(pool, auth_user.company_id, &payload.customer_code)
                .await?
                .is_some()
        }
        DbPool::Sqlite(pool) => {
            Customer::find_by_code_sqlite(pool, auth_user.company_id, &payload.customer_code)
                .await?
                .is_some()
        }
    };
    
    if code_exists {
        tracing::warn!(
            customer_code = %payload.customer_code,
            "Customer creation failed: code already exists"
        );
        return Err(AppError::DuplicateKey("Customer code already exists".to_string()));
    }
    
    // Create customer
    let customer = match state.db.as_ref() {
        DbPool::Postgres(pool) => Customer::create_pg(pool, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => Customer::create_sqlite(pool, payload, auth_user.id).await?,
    };
    
    tracing::info!(
        customer_id = %customer.id,
        name = %customer.name,
        "Customer created successfully"
    );
    
    Ok(created("Customer created successfully", customer))
}

// ===== UPDATE CUSTOMER ENDPOINT =====
#[utoipa::path(
    put,
    path = "/api/v1/customers/{id}",
    tag = "customers",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Customer ID")
    ),
    request_body = UpdateCustomerRequest,
    responses(
        (status = 200, description = "Customer updated successfully", body = Customer),
        (status = 404, description = "Customer not found"),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn update_customer(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCustomerRequest>,
) -> Result<Json<Customer>> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        customer_id = %id,
        "Updating customer"
    );
    
    // Fetch existing customer
    let existing_customer = match state.db.as_ref() {
        DbPool::Postgres(pool) => Customer::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => Customer::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Customer not found".to_string()))?;
    
    // Verify customer belongs to same company
    verify_company_access(&auth_user, existing_customer.company_id)?;
    
    // Update customer
    let updated_customer = match state.db.as_ref() {
        DbPool::Postgres(pool) => Customer::update_pg(pool, id, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => Customer::update_sqlite(pool, id, payload, auth_user.id).await?,
    };
    
    tracing::info!(
        customer_id = %id,
        "Customer updated successfully"
    );
    
    Ok(Json(updated_customer))
}

// ===== DELETE CUSTOMER ENDPOINT =====
#[utoipa::path(
    delete,
    path = "/api/v1/customers/{id}",
    tag = "customers",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Customer ID")
    ),
    responses(
        (status = 204, description = "Customer deleted successfully"),
        (status = 404, description = "Customer not found"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn delete_customer(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        customer_id = %id,
        "Deleting customer"
    );
    
    // Fetch customer to verify company ownership
    let customer = match state.db.as_ref() {
        DbPool::Postgres(pool) => Customer::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => Customer::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Customer not found".to_string()))?;
    
    // Verify customer belongs to same company
    verify_company_access(&auth_user, customer.company_id)?;
    
    // Soft delete customer
    match state.db.as_ref() {
        DbPool::Postgres(pool) => Customer::delete_pg(pool, id).await?,
        DbPool::Sqlite(pool) => Customer::delete_sqlite(pool, id).await?,
    };
    
    tracing::info!(
        customer_id = %id,
        name = %customer.name,
        "Customer deleted successfully"
    );
    
    Ok(no_content())
}

pub fn customers_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::{delete, get, post, put};

    axum::Router::new()
        .route("/", get(list_customers).post(create_customer))
        .route("/:id", get(get_customer).put(update_customer).delete(delete_customer))
}