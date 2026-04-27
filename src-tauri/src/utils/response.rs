// src/utils/response.rs
// Standardized API response structures
// Provides consistent JSON response format across all endpoints

use serde::{Deserialize, Serialize};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

// ===== SUCCESS RESPONSE STRUCTURE =====
// Standard format for successful API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    // HTTP status code
    pub status: u16,
    
    // Success message
    pub message: String,
    
    // Response data (generic type)
    pub data: T,
    
    // Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// ===== PAGINATED RESPONSE STRUCTURE =====
// Response format for paginated data
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    // HTTP status code
    pub status: u16,
    
    // Current page data
    pub data: Vec<T>,
    
    // Pagination metadata
    pub pagination: PaginationMeta,
    
    // Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// ===== PAGINATION METADATA =====
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationMeta {
    // Current page number (1-based)
    pub current_page: i64,
    
    // Number of items per page
    pub per_page: i64,
    
    // Total number of items
    pub total_items: i64,
    
    // Total number of pages
    pub total_pages: i64,
    
    // Has next page
    pub has_next: bool,
    
    // Has previous page
    pub has_previous: bool,
}

// ===== API RESPONSE ENUM =====
// Wrapper for all response types
#[derive(Debug)]
pub enum ApiResponse<T> {
    Success(SuccessResponse<T>),
    Paginated(PaginatedResponse<T>),
}

// ===== IMPLEMENT INTORESPONSE FOR APIRESPONSE =====
impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Success(response) => {
                (StatusCode::OK, Json(response)).into_response()
            },
            ApiResponse::Paginated(response) => {
                (StatusCode::OK, Json(response)).into_response()
            },
        }
    }
}

// ===== HELPER FUNCTIONS =====

// Create success response with data
pub fn success<T>(message: &str, data: T) -> ApiResponse<T>
where
    T: Serialize,
{
    ApiResponse::Success(SuccessResponse {
        status: 200,
        message: message.to_string(),
        data,
        timestamp: chrono::Utc::now(),
    })
}

// Create success response with custom status
pub fn success_with_status<T>(status: StatusCode, message: &str, data: T) -> impl IntoResponse
where
    T: Serialize,
{
    let response = SuccessResponse {
        status: status.as_u16(),
        message: message.to_string(),
        data,
        timestamp: chrono::Utc::now(),
    };
    
    (status, Json(response))
}

// Create paginated response
pub fn paginated<T>(
    data: Vec<T>,
    current_page: i64,
    per_page: i64,
    total_items: i64,
) -> ApiResponse<T>
where
    T: Serialize,
{
    let total_pages = (total_items as f64 / per_page as f64).ceil() as i64;
    
    ApiResponse::Paginated(PaginatedResponse {
        status: 200,
        data,
        pagination: PaginationMeta {
            current_page,
            per_page,
            total_items,
            total_pages,
            has_next: current_page < total_pages,
            has_previous: current_page > 1,
        },
        timestamp: chrono::Utc::now(),
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
        
        match response {
            ApiResponse::Success(resp) => {
                assert_eq!(resp.status, 200);
                assert_eq!(resp.message, "Success");
                assert_eq!(resp.data.id, 1);
            },
            _ => panic!("Expected Success response"),
        }
    }
    
    #[test]
    fn test_paginated_response() {
        let data = vec![
            TestData { id: 1, name: "Item 1".to_string() },
            TestData { id: 2, name: "Item 2".to_string() },
        ];
        
        let response = paginated(data, 1, 10, 100);
        
        match response {
            ApiResponse::Paginated(resp) => {
                assert_eq!(resp.pagination.current_page, 1);
                assert_eq!(resp.pagination.per_page, 10);
                assert_eq!(resp.pagination.total_items, 100);
                assert_eq!(resp.pagination.total_pages, 10);
                assert!(resp.pagination.has_next);
                assert!(!resp.pagination.has_previous);
            },
            _ => panic!("Expected Paginated response"),
        }
    }
}