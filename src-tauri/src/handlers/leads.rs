// src/handlers/leads.rs
// Lead management endpoints
// CRUD operations for leads with status tracking

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
    models::lead::{
        CreateLeadRequest, Lead, LeadSqlite, LeadWithDetails, LeadStatus, UpdateLeadRequest,
    },
    utils::{
        error::{AppError, Result},
        response::{created, no_content, paginated},
    },
    AppState,
};

use super::users::PaginationParams;

// ===== LEAD SEARCH PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct LeadSearchParams {
    pub search: Option<String>,
    
    pub status: Option<LeadStatus>,
    
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// ===== LIST LEADS ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/leads",
    tag = "leads",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("search" = Option<String>, Query, description = "Search term"),
        ("status" = Option<LeadStatus>, Query, description = "Filter by status"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
    ),
    responses(
        (status = 200, description = "List of leads", body = Vec<Lead>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_leads(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<LeadSearchParams>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        "Listing leads"
    );
    
    let (leads, total_count) = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let leads = Lead::list_by_company_pg(
                pool,
                auth_user.company_id,
                params.status.clone(),
                params.pagination.limit(),
                params.pagination.offset(),
            )
            .await?;
            
            let total_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM leads WHERE company_id = $1"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            (leads, total_count)
        }
        DbPool::Sqlite(pool) => {
            let leads = LeadSqlite::list_by_company_sqlite(
                pool,
                auth_user.company_id,
                params.status.clone(),
                params.pagination.limit(),
                params.pagination.offset(),
            )
            .await?;
            
            let total_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM leads WHERE company_id = ?"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            (leads, total_count)
        }
    };
    
    Ok(paginated(
        leads,
        params.pagination.page(),
        params.pagination.per_page(),
        total_count,
    ))
}

// ===== GET LEAD ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/leads/{id}",
    tag = "leads",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Lead ID")
    ),
    responses(
        (status = 200, description = "Lead found", body = LeadWithDetails),
        (status = 404, description = "Lead not found"),
        (status = 403, description = "Access denied")
    )
)]
pub async fn get_lead(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<LeadWithDetails>> {
    let lead = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            Lead::find_by_id_pg(pool, id).await?
        }
        DbPool::Sqlite(pool) => {
            let lead = LeadSqlite::find_by_id_sqlite(pool, id).await?;
            lead.map(|l| l.into())
        }
    }
    .ok_or_else(|| AppError::NotFound("Lead not found".to_string()))?;
    
    verify_company_access(&auth_user, lead.company_id)?;
    
    Ok(Json(LeadWithDetails {
        lead,
        assigned_to_name: None,
        created_by_name: None,
    }))
}

// ===== CREATE LEAD ENDPOINT =====
#[utoipa::path(
    post,
    path = "/api/v1/leads",
    tag = "leads",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateLeadRequest,
    responses(
        (status = 201, description = "Lead created successfully", body = Lead),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_lead(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(mut payload): Json<CreateLeadRequest>,
) -> Result<impl axum::response::IntoResponse> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    payload.company_id = auth_user.company_id;
    
    let lead = match state.db.as_ref() {
        DbPool::Postgres(pool) => Lead::create_pg(pool, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => {
            // TODO: Implement SQLite create
            return Err(AppError::InternalError("SQLite not yet implemented for leads".to_string()));
        }
    };
    
    Ok(created("Lead created successfully", lead))
}

// ===== UPDATE LEAD ENDPOINT =====
#[utoipa::path(
    put,
    path = "/api/v1/leads/{id}",
    tag = "leads",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Lead ID")
    ),
    request_body = UpdateLeadRequest,
    responses(
        (status = 200, description = "Lead updated successfully", body = Lead),
        (status = 404, description = "Lead not found"),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn update_lead(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateLeadRequest>,
) -> Result<Json<Lead>> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    let existing_lead = match state.db.as_ref() {
        DbPool::Postgres(pool) => Lead::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => {
            let lead = LeadSqlite::find_by_id_sqlite(pool, id).await?;
            lead.map(|l| l.into())
        }
    }
    .ok_or_else(|| AppError::NotFound("Lead not found".to_string()))?;
    
    verify_company_access(&auth_user, existing_lead.company_id)?;
    
    let updated_lead = match state.db.as_ref() {
        DbPool::Postgres(pool) => Lead::update_pg(pool, id, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => {
            return Err(AppError::InternalError("SQLite not yet implemented for leads".to_string()));
        }
    };
    
    Ok(Json(updated_lead))
}

// ===== DELETE LEAD ENDPOINT =====
#[utoipa::path(
    delete,
    path = "/api/v1/leads/{id}",
    tag = "leads",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Lead ID")
    ),
    responses(
        (status = 204, description = "Lead deleted successfully"),
        (status = 404, description = "Lead not found"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn delete_lead(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    let lead = match state.db.as_ref() {
        DbPool::Postgres(pool) => Lead::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => {
            let lead = LeadSqlite::find_by_id_sqlite(pool, id).await?;
            lead.map(|l| l.into())
        }
    }
    .ok_or_else(|| AppError::NotFound("Lead not found".to_string()))?;
    
    verify_company_access(&auth_user, lead.company_id)?;
    
    match state.db.as_ref() {
        DbPool::Postgres(pool) => Lead::delete_pg(pool, id).await?,
        DbPool::Sqlite(pool) => {
            return Err(AppError::InternalError("SQLite not yet implemented for leads".to_string()));
        }
    };
    
    Ok(no_content())
}

// ===== LEADS ROUTER =====
pub fn leads_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::{delete, get, post, put};
    
    axum::Router::new()
        .route("/", get(list_leads).post(create_lead))
        .route("/:id", get(get_lead).put(update_lead).delete(delete_lead))
}
