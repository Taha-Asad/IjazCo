// src/utils/response.rs
// Standardized API response structures
// Provides consistent JSON response format across all endpoints

use crate::utils::error::AppError;

use serde::{Deserialize, Serialize};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};

// ===== SUCCESS RESPONSE STRUCTURE =====
// Standard format for successful API responses
// This matches the frontend ApiResponse<T> interface
#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    // Success flag
    pub success: bool,
    
    // Success message
    pub message: String,
    
    // Response data (generic type)
    pub data: T,
}

// ===== PAGINATED RESPONSE STRUCTURE =====
// Response format for paginated data
// This matches the frontend PaginatedResponse<T> interface
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    // Current page data
    pub data: Vec<T>,
    
    // Pagination metadata (flattened to match frontend)
    pub current_page: i64,
    
    // Number of items per page
    pub per_page: i64,
    
    // Total number of items
    pub total_items: i64,
    
    // Total number of pages
    pub total_pages: i64,
}



// ===== API RESPONSE TYPE ALIAS =====
// Type alias for consistent response handling
pub type ApiResponse<T> = Result<Json<SuccessResponse<T>>, AppError>;

// ===== PAGINATED RESPONSE TYPE =====
// Direct return type for paginated endpoints
// This matches the frontend PaginatedResponse<T> interface
pub type PaginatedApiResponse<T> = Result<Json<PaginatedResponse<T>>, AppError>;

// ===== HELPER FUNCTIONS =====

// Create success response with data
pub fn success<T>(message: &str, data: T) -> Json<SuccessResponse<T>>
where
    T: Serialize,
{
    Json(SuccessResponse {
        success: true,
        message: message.to_string(),
        data,
    })
}

// Create success response with custom status
pub fn success_with_status<T>(status: StatusCode, message: &str, data: T) -> impl IntoResponse
where
    T: Serialize,
{
    (status, Json(SuccessResponse {
        success: true,
        message: message.to_string(),
        data,
    }))
}

// Create paginated response
pub fn paginated<T>(
    data: Vec<T>,
    current_page: i64,
    per_page: i64,
    total_items: i64,
) -> Json<PaginatedResponse<T>>
where
    T: Serialize,
{
    let total_pages = (total_items as f64 / per_page as f64).ceil() as i64;
    
    Json(PaginatedResponse {
        data,
        current_page,
        per_page,
        total_items,
        total_pages,
    })
}

// Create "Created" response (201)
pub fn created<T>(message: &str, data: T) -> impl IntoResponse
where
    T: Serialize,
{
    success_with_status(StatusCode::CREATED, message, data)
}

// Create "No Content" response (204)
pub fn no_content() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

// ===== UNIT TESTS =====
#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Serialize, Deserialize)]
    struct TestData {
        id: i32,
        name: String,
    }
    
    #[test]
    fn test_success_response() {
        let data = TestData {
            id: 1,
            name: "Test".to_string(),
        };
        
        let response = success("Success", data);
        let inner = response.0;
        assert_eq!(inner.success, true);
        assert_eq!(inner.message, "Success");
        assert_eq!(inner.data.id, 1);
    }
    
    #[test]
    fn test_paginated_response() {
        let data = vec![
            TestData { id: 1, name: "Item 1".to_string() },
            TestData { id: 2, name: "Item 2".to_string() },
        ];
        
        let response = paginated(data, 1, 10, 100);
        let inner = response.0;
        assert_eq!(inner.current_page, 1);
        assert_eq!(inner.per_page, 10);
        assert_eq!(inner.total_items, 100);
        assert_eq!(inner.total_pages, 10);
    }
}