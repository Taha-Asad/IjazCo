// tests/integration/sales_api_test.rs
// Integration tests for sales workflow

#[cfg(test)]
mod sales_api_tests {
    use std::sync::Arc;
    use axum::{
        body::Body,
        body::to_bytes,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use serde_json::json;
    use uuid::Uuid;
    
    use crate::tests::common::*;
    use crate::create_router;
    use sqlx::Sqlite;
    use sqlx::types::Decimal;

    #[tokio::test]
    async fn test_create_sales_invoice() {
        let state = setup_test_app_state().await;
        let pool = get_sqlite_pool(&state);
        
        let company_id = create_test_company(pool).await;
        let role_id = create_test_role(pool, company_id, UserRole::Admin).await;
        let user_id = create_test_user(pool, company_id, role_id, "salesuser").await;
        let branch_id = create_test_branch(pool, company_id, "SALES").await;
        let customer_id = create_test_customer(pool, company_id, "CUST001").await;
        let item_id = create_test_item(pool, company_id, "SALE-ITEM").await;
        
        // Ensure stock exists
        create_test_stock(pool, company_id, item_id, branch_id, 50).await;
        
        let token = generate_test_token(user_id, company_id, role_id, &get_jwt_secret(&state));
        let app = create_router(state);
        
        let request = Request::builder()
            .uri("/api/v1/sales/invoices")
            .method("POST")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {}", token))
            .body(Body::from(
                json!({
                    "customer_id": customer_id,
                    "branch_id": branch_id,
                    "invoice_date": chrono::Utc::now().date_naive(),
                    "items": [
                        {
                            "item_id": item_id,
                            "quantity": 5,
                            "unit_price": "200.00",
                            "discount_percent": "0"
                        }
                    ],
                    "notes": "Test invoice"
                })
                .to_string(),
            ))
            .unwrap();
        
        let response = app.oneshot(request).await.unwrap();
        
        assert_eq!(response.status(), StatusCode::CREATED);
        
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(json["data"]["status"], "draft");
        assert!(json["data"]["invoice_number"].as_str().is_some());
    }

    #[tokio::test]
    async fn test_approve_sales_invoice() {
        let state = setup_test_app_state().await;
        let pool = get_sqlite_pool(&state);
        
        let company_id = create_test_company(pool).await;
        let role_id = create_test_role(pool, company_id, UserRole::Admin).await;
        let user_id = create_test_user(pool, company_id, role_id, "approveuser").await;
        let branch_id = create_test_branch(pool, company_id, "SALES").await;
        let customer_id = create_test_customer(pool, company_id, "CUST002").await;
        let item_id = create_test_item(pool, company_id, "APPROVE-ITEM").await;
        
        create_test_stock(pool, company_id, item_id, branch_id, 50).await;
        
        // Create invoice first
        let invoice_id = Uuid::new_v4();
        sqlx::query::<Sqlite>(
            r#"INSERT INTO sales_invoices (id, company_id, customer_id, branch_id, 
                invoice_number, invoice_date, status, total_amount, balance_due)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#
        )
        .bind(invoice_id)
        .bind(company_id)
        .bind(customer_id)
        .bind(branch_id)
        .bind("INV-TEST-001")
        .bind(chrono::Utc::now().date_naive())
        .bind("draft")
        .bind("1000")
        .bind("1000")
        .execute(pool)
        .await
        .unwrap();
        
        let app = create_router(Arc::clone(&state));
        let token = generate_test_token(user_id, company_id, role_id, &get_jwt_secret(&state));
        
        // Approve invoice
        let request = Request::builder()
            .uri(format!("/api/v1/sales/invoices/{}/approve", invoice_id))
            .method("POST")
            .header("authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();
        
        let response = app.oneshot(request).await.unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        
        // Verify status changed
        let status: String = sqlx::query_scalar(
            "SELECT status::text FROM sales_invoices WHERE id = $1"
        )
        .bind(invoice_id)
        .fetch_one(pool)
        .await
        .unwrap();
        
        assert_eq!(status, "approved");
    }
}