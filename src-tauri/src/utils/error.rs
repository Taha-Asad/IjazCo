// src/utils/error.rs
// Custom error handling with detailed error types

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::types::Decimal;
use std::fmt;
use validator::ValidationErrors;
use utoipa::ToSchema;
// ===== CUSTOM RESULT TYPE =====
pub type Result<T> = std::result::Result<T, AppError>;

// ===== APPLICATION ERROR ENUM =====
#[derive(Debug)]
pub enum AppError {
    // Database errors
    DatabaseError(sqlx::Error),
    NotFound(String),
    DuplicateKey(String),
    ForeignKeyViolation(String),
    
    // Authentication errors
    InvalidCredentials,
    InvalidToken,
    MissingToken,
    TokenExpired,
    AccountLocked,
    AccountInactive,
    EmailNotVerified,
    
    // Authorization errors
    Forbidden(String),
    InsufficientRole,
    
    // Validation errors
    ValidationError(String),
    BadRequest(String),
    MissingField(String),
    
    // Business logic errors
    InsufficientStock {
        item_id: uuid::Uuid,
        available: i32,
        requested: i32,
    },
    
    // Internal errors
    Internal(String),
    InvalidStatus {
        entity: String,
        current_status: String,
        allowed_statuses: Vec<String>,
    },
    CreditLimitExceeded {
        customer_id: uuid::Uuid,
        limit: Decimal,
        requested: Decimal,
    },
    OperationNotAllowed(String),
    
    // File errors
    FileUploadError(String),
    FileTooLarge {
        size: usize,
        max_size: usize,
    },
    InvalidFileType(String),
    
    // External service errors
    ExternalServiceError(String),
    HttpError(reqwest::Error),
    
    // General errors
    InternalError(String),
    ConfigError(String),
    SerializationError(serde_json::Error),
}

// ===== ERROR RESPONSE STRUCTURE =====
#[derive(Debug, Serialize, Deserialize , ToSchema)]
pub struct ErrorResponse {
    pub status: u16,
    pub error_code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// ===== IMPLEMENT DISPLAY FOR ERROR =====
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::NotFound(entity) => write!(f, "{} not found", entity),
            AppError::DuplicateKey(field) => write!(f, "Duplicate value for {}", field),
            AppError::ForeignKeyViolation(msg) => write!(f, "Foreign key violation: {}", msg),
            AppError::InvalidCredentials => write!(f, "Invalid username or password"),
            AppError::InvalidToken => write!(f, "Invalid authentication token"),
            AppError::MissingToken => write!(f, "Authentication token is missing"),
            AppError::TokenExpired => write!(f, "Authentication token has expired"),
            AppError::AccountLocked => write!(f, "Account is locked"),
            AppError::AccountInactive => write!(f, "Account is inactive"),
            AppError::EmailNotVerified => write!(f, "Email address not verified"),
            AppError::Forbidden(msg) => write!(f, "Access forbidden: {}", msg),
            AppError::InsufficientRole => write!(f, "Insufficient role permissions"),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::MissingField(field) => write!(f, "Missing required field: {}", field),
            AppError::InsufficientStock { item_id, available, requested } => {
                write!(f, "Insufficient stock for item {}: available={}, requested={}", 
                       item_id, available, requested)
            },
            AppError::InvalidStatus { entity, current_status, allowed_statuses } => {
                write!(f, "{} is in status '{}', allowed statuses: {:?}", 
                       entity, current_status, allowed_statuses)
            },
            AppError::CreditLimitExceeded { customer_id, limit, requested } => {
                write!(f, "Credit limit exceeded for customer {}: limit={}, requested={}", 
                       customer_id, limit, requested)
            },
            AppError::OperationNotAllowed(msg) => write!(f, "Operation not allowed: {}", msg),
            AppError::FileUploadError(msg) => write!(f, "File upload error: {}", msg),
            AppError::FileTooLarge { size, max_size } => {
                write!(f, "File too large: {} bytes (max: {} bytes)", size, max_size)
            },
            AppError::InvalidFileType(msg) => write!(f, "Invalid file type: {}", msg),
            AppError::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
            AppError::HttpError(e) => write!(f, "HTTP error: {}", e),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            AppError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// ===== IMPLEMENT AXUM INTORESPONSE FOR ERROR =====
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code) = match &self {
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "BAD_REQUEST"),
            AppError::MissingField(_) => (StatusCode::BAD_REQUEST, "MISSING_FIELD"),
            AppError::InvalidFileType(_) => (StatusCode::BAD_REQUEST, "INVALID_FILE_TYPE"),
            AppError::FileTooLarge { .. } => (StatusCode::BAD_REQUEST, "FILE_TOO_LARGE"),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "INVALID_CREDENTIALS"),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "INVALID_TOKEN"),
            AppError::MissingToken => (StatusCode::UNAUTHORIZED, "MISSING_TOKEN"),
            AppError::TokenExpired => (StatusCode::UNAUTHORIZED, "TOKEN_EXPIRED"),
            AppError::Forbidden(_) => (StatusCode::FORBIDDEN, "FORBIDDEN"),
            AppError::InsufficientRole => (StatusCode::FORBIDDEN, "INSUFFICIENT_ROLE"),
            AppError::AccountLocked => (StatusCode::FORBIDDEN, "ACCOUNT_LOCKED"),
            AppError::AccountInactive => (StatusCode::FORBIDDEN, "ACCOUNT_INACTIVE"),
            AppError::EmailNotVerified => (StatusCode::FORBIDDEN, "EMAIL_NOT_VERIFIED"),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            AppError::DuplicateKey(_) => (StatusCode::CONFLICT, "DUPLICATE_KEY"),
            AppError::InsufficientStock { .. } => (StatusCode::CONFLICT, "INSUFFICIENT_STOCK"),
            AppError::CreditLimitExceeded { .. } => (StatusCode::CONFLICT, "CREDIT_LIMIT_EXCEEDED"),
            AppError::InvalidStatus { .. } => (StatusCode::CONFLICT, "INVALID_STATUS"),
            AppError::ForeignKeyViolation(_) => (StatusCode::UNPROCESSABLE_ENTITY, "FOREIGN_KEY_VIOLATION"),
            AppError::OperationNotAllowed(_) => (StatusCode::UNPROCESSABLE_ENTITY, "OPERATION_NOT_ALLOWED"),
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR"),
            AppError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
            AppError::ConfigError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "CONFIG_ERROR"),
            AppError::SerializationError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "SERIALIZATION_ERROR"),
            AppError::FileUploadError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "FILE_UPLOAD_ERROR"),
            AppError::ExternalServiceError(_) => (StatusCode::BAD_GATEWAY, "EXTERNAL_SERVICE_ERROR"),
            AppError::HttpError(_) => (StatusCode::BAD_GATEWAY, "HTTP_ERROR"),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
        };
        
        let error_response = ErrorResponse {
            status: status.as_u16(),
            error_code: error_code.to_string(),
            message: self.to_string(),
            details: self.get_details(),
            timestamp: chrono::Utc::now(),
        };
        
        tracing::error!(
            status = %status,
            error_code = %error_code,
            message = %error_response.message,
            "API error occurred"
        );
        
        (status, Json(error_response)).into_response()
    }
}

// ===== HELPER METHOD FOR ERROR DETAILS =====
impl AppError {
    fn get_details(&self) -> Option<serde_json::Value> {
        match self {
            AppError::InsufficientStock { item_id, available, requested } => {
                Some(serde_json::json!({
                    "item_id": item_id,
                    "available_quantity": available,
                    "requested_quantity": requested,
                }))
            },
            AppError::InvalidStatus { entity, current_status, allowed_statuses } => {
                Some(serde_json::json!({
                    "entity": entity,
                    "current_status": current_status,
                    "allowed_statuses": allowed_statuses,
                }))
            },
            AppError::CreditLimitExceeded { customer_id, limit, requested } => {
                Some(serde_json::json!({
                    "customer_id": customer_id,
                    "credit_limit": limit,
                    "requested_amount": requested,
                }))
            },
            AppError::FileTooLarge { size, max_size } => {
                Some(serde_json::json!({
                    "file_size_bytes": size,
                    "max_size_bytes": max_size,
                }))
            },
            _ => None,
        }
    }
}

// ===== ERROR CONVERSION IMPLEMENTATIONS =====
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource".to_string()),
            sqlx::Error::Database(db_err) => {
                let error_msg = db_err.message();
                if error_msg.contains("unique") || error_msg.contains("duplicate") {
                    AppError::DuplicateKey(error_msg.to_string())
                } else if error_msg.contains("foreign key") {
                    AppError::ForeignKeyViolation(error_msg.to_string())
                } else {
                    AppError::DatabaseError(sqlx::Error::Database(db_err))
                }
            },
            _ => AppError::DatabaseError(err),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerializationError(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::HttpError(err)
    }
}

impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        let message = err
            .field_errors()
            .iter()
            .next()
            .and_then(|(field, errors)| {
                errors.first().map(|e| {
                    format!("{}: {}", field, e.message.as_ref().unwrap_or(&std::borrow::Cow::Borrowed("validation failed")))
                })
            })
            .unwrap_or_else(|| "Validation failed".to_string());
        
        AppError::ValidationError(message)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind;
        match err.kind() {
            ErrorKind::ExpiredSignature => AppError::TokenExpired,
            _ => AppError::InvalidToken,
        }
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::InternalError(format!("Password hashing error: {}", err))
    }
}