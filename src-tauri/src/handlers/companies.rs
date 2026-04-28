// src/handlers/companies.rs
use axum::{extract::{Path, Query, State}, Json, response::IntoResponse};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use serde::Deserialize;

use crate::{
    config::{AppState, DbPool},
    middleware::auth::AuthUser,
    models::company::{Company, CreateCompanyRequest, UpdateCompanyRequest},
    utils::{error::{AppError, Result}, response::{created, no_content, success}},
};

use super::users::PaginationParams;

#[derive(Debug, Deserialize)]
pub struct ListCompaniesQuery {
    pub active_only: Option<bool>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn list_companies(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Query(params): Query<ListCompaniesQuery>,
) -> Result<impl axum::response::IntoResponse> {
    let companies = match state.db.as_ref() {
        DbPool::Postgres(pool) => Company::list_all_pg(pool).await?,
        DbPool::Sqlite(pool) => Company::list_all_sqlite(pool).await?,
    };

    let filtered: Vec<Company> = if params.active_only.unwrap_or(false) {
        companies.into_iter().filter(|c| c.is_active).collect()
    } else {
        companies
    };

    Ok(success("Companies retrieved", filtered))
}

pub async fn get_company(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Company>> {
    let company = match state.db.as_ref() {
        DbPool::Postgres(pool) => Company::find_by_id_pg(&pool, id).await?,
        DbPool::Sqlite(pool) => Company::find_by_id_sqlite(&pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Company not found".to_string()))?;

    Ok(Json(company))
}

pub async fn create_company(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(payload): Json<CreateCompanyRequest>,
) -> Result<impl IntoResponse> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let company = match state.db.as_ref() {
        DbPool::Postgres(pool) => Company::create_pg(&pool, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => Company::create_sqlite(&pool, payload, auth_user.id).await?,
    };

    Ok(created("Company created", company))
}

pub async fn update_company(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCompanyRequest>,
) -> Result<Json<Company>> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let company = match state.db.as_ref() {
        DbPool::Postgres(pool) => Company::update_pg(&pool, id, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => Company::update_sqlite(&pool, id, payload, auth_user.id).await?,
    };

    Ok(Json(company))
}

pub async fn delete_company(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    match state.db.as_ref() {
        DbPool::Postgres(pool) => Company::delete_pg(&pool, id, auth_user.id).await?,
        DbPool::Sqlite(pool) => Company::delete_sqlite(&pool, id, auth_user.id).await?,
    };

    Ok(no_content())
}

pub fn companies_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::{delete, get, post, put};
    axum::Router::new()
        .route("/", get(list_companies).post(create_company))
        .route("/:id", get(get_company).put(update_company).delete(delete_company))
}