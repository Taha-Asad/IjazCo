// tests/integration/auth_api_test.rs (Continued)
// Integration tests for authentication endpoints

#[cfg(test)]
mod auth_api_tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use serde_json::json;
    
    use crate::common::*;

    // ... [Previous tests from previous prompt] ...

    #[tokio::test]
    async fn test_protected_endpoint_with_valid_token() {
        let state = setup_test_app_state().await;
        let pool = state.sqlite_pool.as_ref().unwrap();
        
        let company_id = create_test_company(pool).await;
        let role_id = create_test_role(pool, company_id, UserRole::Admin).await;
        let user_id = create_test_user(pool, company_id, role_id, "authuser").await;
        
        // Generate valid token
        let token = generate_test_token(
            user_id,
            company_id,
            role_id,
            &state.jwt_secret,
        );
        
        // Build router using exposed test function
        let app = erp_backend::create_test_router(state);
        
        // Access protected endpoint with token
        let request = Request::builder()
            .uri("/api/v1/users")
            .method("GET")
            .header("authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();
        
        let response = app.oneshot(request).await.unwrap();
        
        // Assert: Success (200 OK)
        assert_eq!(response.status(), StatusCode::OK);
        
        // Verify response contains users list (even if empty)
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert!(json.get("data").is_some());
        assert!(json.get("success").is_some());
    }

    #[tokio::test]
    async fn test_refresh_token_flow() {
        let state = setup_test_app_state().await;
        let pool = state.sqlite_pool.as_ref().unwrap();
        
        let company_id = create_test_company(pool).await;
        let role_id = create_test_role(pool, company_id, UserRole::Admin).await;
        let _ = create_test_user(pool, company_id, role_id, "refreshtest").await;
        
        let app = erp_backend::create_test_router(state.clone());
        
        // 1. Login to get tokens
        let login_req = Request::builder()
            .uri("/api/v1/auth/login")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(
                json!({
                    "username": "refreshtest",
                    "password": "TestPass123!",
                    "remember_me": false
                })
                .to_string(),
            ))
            .unwrap();
        
        let login_resp = app.oneshot(login_req).await.unwrap();
        assert_eq!(login_resp.status(), StatusCode::OK);
        
        let body = hyper::body::to_bytes(login_resp.into_body()).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let refresh_token = json["refresh_token"].as_str().unwrap();
        
        // 2. Use refresh token to get new access token
        let refresh_req = Request::builder()
            .uri("/api/v1/auth/refresh")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(
                json!({
                    "refresh_token": refresh_token
                })
                .to_string(),
            ))
            .unwrap();
        
        let refresh_resp = app.oneshot(refresh_req).await.unwrap();
        assert_eq!(refresh_resp.status(), StatusCode::OK);
        
        let body = hyper::body::to_bytes(refresh_resp.into_body()).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert!(json["access_token"].as_str().is_some());
    }
}