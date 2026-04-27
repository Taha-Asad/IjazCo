// src/utils/mod.rs
// Utility functions module - exports all utility submodules

pub mod error;       // Custom error types and handling
pub mod jwt;         // JWT token generation and validation
pub mod password;    // Password hashing and verification
pub mod validator;   // Custom validation functions
pub mod response;    // API response helpers

// Re-export commonly used utilities
pub use error::{AppError, Result};
pub use jwt::{generate_jwt, validate_jwt, Claims};
pub use password::{hash_password, verify_password};
pub use response::{ApiResponse, SuccessResponse};