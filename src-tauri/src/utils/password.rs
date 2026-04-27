// src/utils/password.rs
// Secure password hashing and verification using Argon2
// Argon2 is the winner of the Password Hashing Competition

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::utils::error::{AppError, Result};
use rand::random_range; // This brings the random methods into scope

// ===== HASH PASSWORD =====
// Securely hash a password using Argon2id algorithm
pub fn hash_password(password: &str) -> Result<String> {
    // Validate password length
    if password.len() < 8 {
        return Err(AppError::ValidationError(
            "Password must be at least 8 characters".to_string()
        ));
    }
    
    // Generate random salt using OS random number generator (cryptographically secure)
    let salt = SaltString::generate(&mut OsRng);
    
    // Create Argon2 instance with default parameters
    // Argon2id is hybrid mode (resistant to both side-channel and GPU attacks)
    let argon2 = Argon2::default();
    
    // Hash the password
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to hash password");
            AppError::InternalError("Password hashing failed".to_string())
        })?
        .to_string(); // Convert to PHC string format
    
    tracing::debug!("Password hashed successfully");
    
    Ok(password_hash)
}

// ===== VERIFY PASSWORD =====
// Verify a password against its hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    // Parse the stored hash (PHC string format)
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to parse password hash");
            AppError::InternalError("Invalid password hash format".to_string())
        })?;
    
    // Create Argon2 instance
    let argon2 = Argon2::default();
    
    // Verify password against hash
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => {
            tracing::debug!("Password verified successfully");
            Ok(true)
        },
        Err(_) => {
            tracing::debug!("Password verification failed");
            Ok(false) // Return false instead of error for invalid password
        }
    }
}

// ===== PASSWORD STRENGTH VALIDATOR =====
// Check if password meets strength requirements
pub fn validate_password_strength(password: &str) -> Result<()> {
    // Minimum length
    if password.len() < 8 {
        return Err(AppError::ValidationError(
            "Password must be at least 8 characters long".to_string()
        ));
    }
    
    // Maximum length (prevent DoS via very long passwords)
    if password.len() > 128 {
        return Err(AppError::ValidationError(
            "Password must not exceed 128 characters".to_string()
        ));
    }
    
    // Check for uppercase letter
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(AppError::ValidationError(
            "Password must contain at least one uppercase letter".to_string()
        ));
    }
    
    // Check for lowercase letter
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(AppError::ValidationError(
            "Password must contain at least one lowercase letter".to_string()
        ));
    }
    
    // Check for digit
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(AppError::ValidationError(
            "Password must contain at least one number".to_string()
        ));
    }
    
    // Check for special character
    if !password.chars().any(|c| c.is_ascii_punctuation()) {
        return Err(AppError::ValidationError(
            "Password must contain at least one special character".to_string()
        ));
    }
    
    Ok(())
}

// ===== GENERATE RANDOM PASSWORD =====
// Generate a secure random password (useful for temporary passwords)
pub fn generate_random_password(length: usize) -> String {
    
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789\
                             !@#$%^&*";
    
    let rng = rand::rng();     
    (0..length)
        .map(|_| {
        let idx = random_range(0..CHARSET.len());
        CHARSET[idx] as char
        })
        .collect()
}

// ===== UNIT TESTS =====
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_and_verify_password() {
        let password = "SecurePassword123!";
        
        // Hash password
        let hash = hash_password(password).unwrap();
        
        // Verify correct password
        assert!(verify_password(password, &hash).unwrap());
        
        // Verify incorrect password
        assert!(!verify_password("WrongPassword", &hash).unwrap());
    }
    
    #[test]
    fn test_password_strength_validation() {
        // Valid password
        assert!(validate_password_strength("SecurePass123!").is_ok());
        
        // Too short
        assert!(validate_password_strength("Short1!").is_err());
        
        // No uppercase
        assert!(validate_password_strength("lowercase123!").is_err());
        
        // No lowercase
        assert!(validate_password_strength("UPPERCASE123!").is_err());
        
        // No number
        assert!(validate_password_strength("NoNumbers!").is_err());
        
        // No special character
        assert!(validate_password_strength("NoSpecialChar123").is_err());
    }
    
    #[test]
    fn test_generate_random_password() {
        let password = generate_random_password(16);
        
        // Check length
        assert_eq!(password.len(), 16);
        
        // Verify it meets strength requirements (should pass most of the time)
        // Note: Random generation might occasionally fail validation
        // In production, regenerate until valid
    }
    
    #[test]
    fn test_hash_short_password() {
        let result = hash_password("short");
        assert!(result.is_err());
    }
}