// tests/unit/user_model_test.rs
// Unit tests for User model
// Tests CRUD operations, password verification, and validation

#[cfg(test)]
mod user_model_tests {
    use crate::common::*;
    
    // ===== CREATE USER TESTS =====
    
    #[tokio::test]
    async fn test_create_user_success() {
        // Setup: Create test database and dependencies
        let pool = setup_test_db().await;
        let company_id = create_test_company(&pool).await;
        let role_id = create_test_role(&pool, company_id, UserRole::Admin).await;
        
        // Create user request
        let request = CreateUserRequest {
            company_id,
            role_id,
            username: "newuser".to_string(),
            email: "newuser@test.com".to_string(),
            password: "SecurePass123!".to_string(),
            first_name: "New".to_string(),
            last_name: "User".to_string(),
            phone: Some("+1-555-1234".to_string()),
        };
        
        // Execute: Create user
        let result = User::create(&pool, request, Uuid::new_v4()).await;
        
        // Assert: User created successfully
        assert!(result.is_ok(), "Failed to create user: {:?}", result.err());
        
        let user = result.unwrap();
        assert_eq!(user.username, "newuser");
        assert_eq!(user.email, "newuser@test.com");
        assert_eq!(user.company_id, company_id);
        assert_eq!(user.status, UserStatus::Pending); // New users start as pending
        
        cleanup_test_db(&pool).await;
    }
    
    #[tokio::test]
    async fn test_create_user_duplicate_username() {
        // Setup
        let pool = setup_test_db().await;
        let company_id = create_test_company(&pool).await;
        let role_id = create_test_role(&pool, company_id, UserRole::Admin).await;
        
        // Create first user
        let _ = create_test_user(&pool, company_id, role_id, "duplicate").await;
        
        // Attempt to create user with same username
        let request = CreateUserRequest {
            company_id,
            role_id,
            username: "duplicate".to_string(),
            email: "different@test.com".to_string(),
            password: "SecurePass123!".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            phone: None,
        };
        
        let result = User::create(&pool, request, Uuid::new_v4()).await;
        
        // Assert: Should fail due to duplicate username
        assert!(result.is_err(), "Expected error for duplicate username");
        
        cleanup_test_db(&pool).await;
    }
    
    // ===== FIND USER TESTS =====
    
    #[tokio::test]
    async fn test_find_user_by_id() {
        // Setup
        let pool = setup_test_db().await;
        let company_id = create_test_company(&pool).await;
        let role_id = create_test_role(&pool, company_id, UserRole::Admin).await;
        let user_id = create_test_user(&pool, company_id, role_id, "findme").await;
        
        // Execute: Find user
        let result = User::find_by_id(&pool, user_id).await;
        
        // Assert: User found
        assert!(result.is_ok());
        let user = result.unwrap();
        assert!(user.is_some());
        assert_eq!(user.unwrap().id, user_id);
        
        cleanup_test_db(&pool).await;
    }
    
    #[tokio::test]
    async fn test_find_user_by_username() {
        // Setup
        let pool = setup_test_db().await;
        let company_id = create_test_company(&pool).await;
        let role_id = create_test_role(&pool, company_id, UserRole::Admin).await;
        let _ = create_test_user(&pool, company_id, role_id, "searchuser").await;
        
        // Execute: Find by username
        let result = User::find_by_username(&pool, "searchuser").await;
        
        // Assert
        assert!(result.is_ok());
        let user = result.unwrap();
        assert!(user.is_some());
        assert_eq!(user.unwrap().username, "searchuser");
        
        cleanup_test_db(&pool).await;
    }
    
    // ===== PASSWORD VERIFICATION TESTS =====
    
    #[tokio::test]
    async fn test_password_verification_success() {
        // Setup
        let pool = setup_test_db().await;
        let company_id = create_test_company(&pool).await;
        let role_id = create_test_role(&pool, company_id, UserRole::Admin).await;
        let user_id = create_test_user(&pool, company_id, role_id, "passtest").await;
        
        // Get user
        let user = User::find_by_id(&pool, user_id).await.unwrap().unwrap();
        
        // Verify correct password
        let result = user.verify_password("TestPass123!");
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        // Verify incorrect password
        let result = user.verify_password("WrongPassword");
        assert!(result.is_ok());
        assert!(!result.unwrap());
        
        cleanup_test_db(&pool).await;
    }
    
    // ===== UPDATE USER TESTS =====
    
    #[tokio::test]
    async fn test_update_user() {
        // Setup
        let pool = setup_test_db().await;
        let company_id = create_test_company(&pool).await;
        let role_id = create_test_role(&pool, company_id, UserRole::Admin).await;
        let user_id = create_test_user(&pool, company_id, role_id, "updateme").await;
        
        // Update request
        let request = crate::models::user::UpdateUserRequest {
            email: Some("newemail@test.com".to_string()),
            first_name: Some("Updated".to_string()),
            last_name: None,
            phone: Some("+1-555-9999".to_string()),
            avatar_url: None,
            role_id: None,
            status: None,
            preferences: None,
        };
        
        // Execute: Update user
        let result = User::update(&pool, user_id, request, Uuid::new_v4()).await;
        
        // Assert
        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.email, "newemail@test.com");
        assert_eq!(updated.first_name, "Updated");
        
        cleanup_test_db(&pool).await;
    }
    
    // ===== DELETE USER TESTS =====
    
    #[tokio::test]
    async fn test_delete_user_soft_delete() {
        // Setup
        let pool = setup_test_db().await;
        let company_id = create_test_company(&pool).await;
        let role_id = create_test_role(&pool, company_id, UserRole::Admin).await;
        let user_id = create_test_user(&pool, company_id, role_id, "deleteme").await;
        
        // Execute: Delete user
        let result = User::delete(&pool, user_id, Uuid::new_v4()).await;
        assert!(result.is_ok());
        
        // Verify: User still in database but marked as deleted
        let user = User::find_by_id(&pool, user_id).await.unwrap();
        assert!(user.is_none()); // Should not find deleted users
        
        cleanup_test_db(&pool).await;
    }
}