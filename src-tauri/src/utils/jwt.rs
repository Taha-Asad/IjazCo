// src/utils/jwt.rs
// JWT token generation and validation
// Handles access tokens and refresh tokens for stateless authentication

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::utils::error::{AppError, Result};

// ===== JWT CLAIMS STRUCTURE =====
// Data stored inside JWT token
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // Subject - user ID
    pub sub: String,
    
    // Company ID (for multi-tenant isolation)
    pub company_id: String,
    
    // User role ID
    pub role_id: String,
    
    // User email
    pub email: String,
    
    // Username
    pub username: String,
    
    // Token type (access or refresh)
    pub token_type: String,
    
    // Issued at timestamp (Unix time)
    pub iat: i64,
    
    // Expiration timestamp (Unix time)
    pub exp: i64,
}

// ===== TOKEN TYPE ENUM =====
#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Access,   // Short-lived token for API access
    Refresh,  // Long-lived token for obtaining new access tokens
}

impl TokenType {
    // Get token expiration duration
    fn expiration_duration(&self) -> Duration {
        match self {
            TokenType::Access => Duration::hours(1),   // 1 hour for access tokens
            TokenType::Refresh => Duration::days(7),   // 7 days for refresh tokens
        }
    }
    
    // Get token type string
    fn as_str(&self) -> &str {
        match self {
            TokenType::Access => "access",
            TokenType::Refresh => "refresh",
        }
    }
}

// ===== GENERATE JWT TOKEN =====
// Creates a new JWT token with user claims
pub fn generate_jwt(
    user_id: Uuid,
    company_id: Uuid,
    role_id: Uuid,
    email: &str,
    username: &str,
    token_type: TokenType,
    secret: &str,
) -> Result<String> {
    // Calculate token expiration time
    let now = Utc::now();
    let expiration = now + token_type.expiration_duration();
    
    // Build claims
    let claims = Claims {
        sub: user_id.to_string(),                    // User ID as subject
        company_id: company_id.to_string(),          // Company for multi-tenancy
        role_id: role_id.to_string(),                // User's role
        email: email.to_string(),                    // User email
        username: username.to_string(),              // Username
        token_type: token_type.as_str().to_string(), // Token type
        iat: now.timestamp(),                        // Issued at (Unix timestamp)
        exp: expiration.timestamp(),                 // Expiration (Unix timestamp)
    };
    
    // Encode JWT token
    let token = encode(
        &Header::default(),                          // Use default header (HS256 algorithm)
        &claims,                                     // Token payload
        &EncodingKey::from_secret(secret.as_ref()),  // Secret key for signing
    )
    .map_err(|e| AppError::InternalError(format!("Failed to generate JWT: {}", e)))?;
    
    tracing::debug!(
        user_id = %user_id,
        token_type = token_type.as_str(),
        expires_at = %expiration,
        "Generated JWT token"
    );
    
    Ok(token)
}

// ===== VALIDATE JWT TOKEN =====
// Verifies and decodes a JWT token
pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims> {
    // Set up validation rules
    let validation = Validation::default(); // Uses HS256 by default
    
    // Decode and validate token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),  // Secret key for verification
        &validation,
    )
    .map_err(|e| {
        tracing::warn!(error = %e, "JWT validation failed");
        AppError::from(e) // Convert JWT error to AppError
    })?;
    
    // Verify token hasn't expired (double-check even though library validates)
    let now = Utc::now().timestamp();
    if token_data.claims.exp < now {
        return Err(AppError::TokenExpired);
    }
    
    tracing::debug!(
        user_id = %token_data.claims.sub,
        token_type = %token_data.claims.token_type,
        "JWT token validated successfully"
    );
    
    Ok(token_data.claims)
}

// ===== EXTRACT TOKEN FROM HEADER =====
// Extracts JWT token from Authorization header
pub fn extract_token_from_header(auth_header: &str) -> Result<&str> {
    // Authorization header format: "Bearer <token>"
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::InvalidToken);
    }
    
    // Extract token part (skip "Bearer " prefix)
    let token = auth_header.trim_start_matches("Bearer ").trim();
    
    if token.is_empty() {
        return Err(AppError::MissingToken);
    }
    
    Ok(token)
}

// ===== REFRESH ACCESS TOKEN =====
// Validates refresh token and generates new access token
pub fn refresh_access_token(
    refresh_token: &str,
    secret: &str,
) -> Result<(String, Claims)> {
    // Validate refresh token
    let claims = validate_jwt(refresh_token, secret)?;
    
    // Verify it's actually a refresh token
    if claims.token_type != "refresh" {
        return Err(AppError::InvalidToken);
    }
    
    // Generate new access token with same user info
    let access_token = generate_jwt(
        Uuid::parse_str(&claims.sub).map_err(|_| AppError::InvalidToken)?,
        Uuid::parse_str(&claims.company_id).map_err(|_| AppError::InvalidToken)?,
        Uuid::parse_str(&claims.role_id).map_err(|_| AppError::InvalidToken)?,
        &claims.email,
        &claims.username,
        TokenType::Access,
        secret,
    )?;
    
    Ok((access_token, claims))
}

// ===== UNIT TESTS =====
#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_SECRET: &str = "test_secret_key_for_jwt_testing_only";
    
    #[test]
    fn test_generate_and_validate_jwt() {
        // Generate test token
        let user_id = Uuid::new_v4();
        let company_id = Uuid::new_v4();
        let role_id = Uuid::new_v4();
        
        let token = generate_jwt(
            user_id,
            company_id,
            role_id,
            "test@example.com",
            "testuser",
            TokenType::Access,
            TEST_SECRET,
        )
        .unwrap();
        
        // Validate token
        let claims = validate_jwt(&token, TEST_SECRET).unwrap();
        
        // Verify claims
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.company_id, company_id.to_string());
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.token_type, "access");
    }
    
    #[test]
    fn test_invalid_token() {
        let result = validate_jwt("invalid.token.here", TEST_SECRET);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_extract_token_from_header() {
        // Valid header
        let header = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        let token = extract_token_from_header(header).unwrap();
        assert_eq!(token, "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9");
        
        // Invalid header (missing Bearer)
        let result = extract_token_from_header("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9");
        assert!(result.is_err());
        
        // Empty token
        let result = extract_token_from_header("Bearer ");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_refresh_token_flow() {
        let user_id = Uuid::new_v4();
        let company_id = Uuid::new_v4();
        let role_id = Uuid::new_v4();
        
        // Generate refresh token
        let refresh_token = generate_jwt(
            user_id,
            company_id,
            role_id,
            "test@example.com",
            "testuser",
            TokenType::Refresh,
            TEST_SECRET,
        )
        .unwrap();
        
        // Use refresh token to get new access token
        let (access_token, claims) = refresh_access_token(&refresh_token, TEST_SECRET).unwrap();
        
        // Verify new access token
        let access_claims = validate_jwt(&access_token, TEST_SECRET).unwrap();
        assert_eq!(access_claims.token_type, "access");
        assert_eq!(access_claims.sub, user_id.to_string());
    }
}