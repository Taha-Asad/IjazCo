// src/handlers/companies.rs
use axum::{extract::{Path, State}, Json, response::IntoResponse};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::{AppState, DbPool},
    middleware::auth::AuthUser,
    models::company::{Company, CreateCompanyRequest, UpdateCompanyRequest},
    utils::{error::{AppError, Result}, response::{created, success}},
};

pub async fn get_company(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Company>> {
    let company = match state.db.as_ref() {
        DbPool::Postgres(pool) => Company::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => Company::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Company not found".to_string()))?;

    Ok(Json(company))
}

pub async fn create_company(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser, // Assumes a system admin is calling
    Json(payload): Json<CreateCompanyRequest>,
) -> Result<impl IntoResponse> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let company = match state.db.as_ref() {
        DbPool::Postgres(pool) => Company::create_pg(pool, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => Company::create_sqlite(pool, payload, auth_user.id).await?,
    };

    Ok(created("Company created", company))
}

pub fn companies_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::{get, post};
    axum::Router::new()
        .route("/", post(create_company))
        .route("/:id", get(get_company))
}