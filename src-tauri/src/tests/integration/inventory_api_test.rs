// tests/integration/inventory_api_test.rs
// Integration tests for inventory management endpoints

#[cfg(test)]
mod inventory_api_tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use serde_json::json;
    use uuid::Uuid;
    
    use crate::common::*;

    #[tokio::test]
    async fn test_create_inventory_item() {
        let state = setup_test_app_state().await;
        let pool = state.sqlite_pool.as_ref().unwrap();
        
        let company_id = create_test_company(pool).await;
        let role_id = create_test_role(pool, company_id, UserRole::Admin).await;
        let user_id = create_test_user(pool, company_id, role_id, "inventoryuser").await;
        let token = generate_test_token(user_id, company_id, role_id, &state.jwt_secret);
        
        let app = erp_backend::create_test_router(state);
        
        let request = Request::builder()
            .uri("/api/v1/inventory/items")
            .method("POST")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {}", token))
            .body(Body::from(
                json!({
                    "sku": "TEST-SKU-001",
                    "name": "Test Product",
                    "description": "A test product",
                    "unit_of_measure": "PCS",
                    "cost_price": "100.00",
                    "selling_price": "150.00",
                    "tax_rate": "10.0",
                    "reorder_level": 10,
                    "is_active": true
                })
                .to_string(),
            ))
            .unwrap();
        
        let response = app.oneshot(request).await.unwrap();
        
        assert_eq!(response.status(), StatusCode::CREATED);
        
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(json["data"]["sku"], "TEST-SKU-001");
        assert_eq!(json["data"]["name"], "Test Product");
    }

    #[tokio::test]
    async fn test_get_inventory_items() {
        let state = setup_test_app_state().await;
        let pool = state.sqlite_pool.as_ref().unwrap();
        
        let company_id = create_test_company(pool).await;
        let role_id = create_test_role(pool, company_id, UserRole::Admin).await;
        let user_id = create_test_user(pool, company_id, role_id, "getitemsuser").await;
        
        // Create test item
        let item_id = create_test_item(pool, company_id, "GET-TEST-SKU").await;
        
        let token = generate_test_token(user_id, company_id, role_id, &state.jwt_secret);
        let app = erp_backend::create_test_router(state);
        
        let request = Request::builder()
            .uri("/api/v1/inventory/items")
            .method("GET")
            .header("authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();
        
        let response = app.oneshot(request).await.unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        let items = json["data"]["items"].as_array().unwrap();
        assert!(!items.is_empty());
        
        // Verify our test item is in the list
        let found = items.iter().any(|item| {
            item["sku"].as_str() == Some("GET-TEST-SKU")
        });
        assert!(found);
    }

    #[tokio::test]
    async fn test_stock_adjustment() {
        let state = setup_test_app_state().await;
        let pool = state.sqlite_pool.as_ref().unwrap();
        
        let company_id = create_test_company(pool).await;
        let role_id = create_test_role(pool, company_id, UserRole::Admin).await;
        let user_id = create_test_user(pool, company_id, role_id, "stockuser").await;
        let branch_id = create_test_branch(pool, company_id, "MAIN").await;
        let item_id = create_test_item(pool, company_id, "STOCK-TEST-SKU").await;
        
        // Initialize stock
        create_test_stock(pool, company_id, item_id, branch_id, 100).await;
        
        let token = generate_test_token(user_id, company_id, role_id, &state.jwt_secret);
        let app = erp_backend::create_test_router(state);
        
        // Adjust stock
        let request = Request::builder()
            .uri("/api/v1/inventory/stock/adjust")
            .method("POST")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {}", token))
            .body(Body::from(
                json!({
                    "item_id": item_id,
                    "branch_id": branch_id,
                    "quantity": -10,
                    "reason": "Damaged goods",
                    "reference": "ADJ-001"
                })
                .to_string(),
            ))
            .unwrap();
        
        let response = app.oneshot(request).await.unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        
        // Verify stock level updated
        let stock_level: i32 = sqlx::query_scalar(
            "SELECT quantity_on_hand FROM stock WHERE item_id = $1 AND branch_id = $2"
        )
        .bind(item_id)
        .bind(branch_id)
        .fetch_one(pool)
        .await
        .unwrap();
        
        assert_eq!(stock_level, 90);
    }
}