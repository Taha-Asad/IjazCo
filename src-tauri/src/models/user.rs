// src/models/user.rs
// User authentication and authorization models

use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use sqlx::{ FromRow, Sqlite, SqlitePool, PgPool, Postgres };
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use lazy_static::lazy_static;
use regex::Regex;


// ===== USER ROLE ENUM =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "inventory_manager")]
    InventoryManager,
    #[serde(rename = "sales_user")]
    SalesUser,
    #[serde(rename = "import_clerk")]
    ImportClerk,
}

// ===== USER STATUS ENUM =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum UserStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "pending")]
    Pending,
}

// ===== USER MODEL =====
#[derive(Debug, Clone, Serialize, PartialEq, Deserialize, FromRow , ToSchema)]
pub struct User {
    pub id: Uuid,
    pub company_id: Uuid,
    pub role_id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub is_email_verified: bool,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub last_login_ip: Option<String>,
    pub failed_login_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    #[serde(skip_serializing)]
    pub password_reset_token: Option<String>,
    #[serde(skip_serializing)]
    pub password_reset_expires_at: Option<DateTime<Utc>>,
    pub two_factor_enabled: bool,
    #[serde(skip_serializing)]
    pub two_factor_secret: Option<String>,
    pub preferences: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
}

// ===== USER SAFE MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserSafe {
    pub id: Uuid,
    pub company_id: Uuid,
    pub role_id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub is_email_verified: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub two_factor_enabled: bool,
    pub preferences: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserSafe {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            company_id: user.company_id,
            role_id: user.role_id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            phone: user.phone,
            avatar_url: user.avatar_url,
            status: user.status,
            is_email_verified: user.is_email_verified,
            last_login_at: user.last_login_at,
            two_factor_enabled: user.two_factor_enabled,
            preferences: user.preferences,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

// ===== CREATE USER REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    #[validate(regex(path = "USERNAME_REGEX"))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 128))]
    pub password: String,

    #[validate(length(min = 1, max = 100))]
    pub first_name: String,

    #[validate(length(min = 1, max = 100))]
    pub last_name: String,

    #[validate(length(max = 20))]
    pub phone: Option<String>,

    pub role_id: Uuid,
    pub company_id: Uuid,
}

lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
}

// ===== UPDATE USER REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema  , Default)]
pub struct UpdateUserRequest {
    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(min = 1, max = 100))]
    pub first_name: Option<String>,

    #[validate(length(min = 1, max = 100))]
    pub last_name: Option<String>,

    #[validate(length(max = 20))]
    pub phone: Option<String>,

    pub avatar_url: Option<String>,
    pub role_id: Option<Uuid>,
    pub status: Option<UserStatus>,
    pub preferences: Option<serde_json::Value>,
}

// ===== CHANGE PASSWORD REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 8))]
    pub current_password: String,

    #[validate(length(min = 8, max = 128))]
    pub new_password: String,

    #[validate(length(min = 8, max = 128))]
    #[validate(must_match = "new_password")]
    pub confirm_password: String,
}

// ===== PASSWORD STRENGTH VALIDATION =====
pub fn validate_password_strength(password: &str) -> Result<(), validator::ValidationError> {
    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_digit = false;
    let mut has_special = false;

    for c in password.chars() {
        if c.is_uppercase() {
            has_uppercase = true;
        } else if c.is_lowercase() {
            has_lowercase = true;
        } else if c.is_digit(10) {
            has_digit = true;
        } else if c.is_ascii_punctuation() {
            has_special = true;
        }
    }

    if has_uppercase && has_lowercase && has_digit && has_special {
        Ok(())
    } else {
        Err(
            validator::ValidationError::new(
                "Password must contain uppercase, lowercase, digit, and special character"
            )
        )
    }
}

// ===== ROLE MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub role_type: UserRole,
    pub permissions: serde_json::Value,
    pub is_system: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ===== CREATE ROLE REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateRoleRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    pub description: Option<String>,
    pub role_type: UserRole,
    pub permissions: serde_json::Value,
    pub company_id: Uuid,
    pub is_system: Option<bool>,
}

// ===== UPDATE ROLE REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema, Default)]
pub struct UpdateRoleRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub permissions: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

// ===== USER DATABASE OPERATIONS =====
impl User {

// inside src/models/user.rs

    pub async fn create_sqlite(
        pool: &SqlitePool,
        request: CreateUserRequest,
        created_by: Option<Uuid>
    ) -> Result<User, sqlx::Error> {
        let password_hash = crate::utils::password
            ::hash_password(&request.password)
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        // Generate the new UUID in Rust
        let new_id = Uuid::new_v4();

        let user = sqlx::query_as::<Sqlite, User>(
            r#"
            INSERT INTO users (
                id, company_id, role_id, username, email, password_hash,
                first_name, last_name, phone, status, is_email_verified,
                failed_login_attempts, two_factor_enabled, preferences,
                created_by, updated_by
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(new_id) // Bind the ID here
        .bind(request.company_id)
        .bind(request.role_id)
        .bind(request.username)
        .bind(request.email)
        .bind(password_hash)
        .bind(request.first_name)
        .bind(request.last_name)
        .bind(request.phone)
        .bind(UserStatus::Pending)
        .bind(false)
        .bind(0)
        .bind(false)
        .bind(serde_json::json!({}))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn create_pg(
        pool: &PgPool,
        request: CreateUserRequest,
        created_by: Option<Uuid>
    ) -> Result<User, sqlx::Error> {
        let password_hash = crate::utils::password
            ::hash_password(&request.password)
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        // Also generate ID in Rust for Postgres to ensure consistency
        let new_id = Uuid::new_v4();

        let user = sqlx::query_as::<Postgres, User>(
            r#"
            INSERT INTO users (
                id, company_id, role_id, username, email, password_hash,
                first_name, last_name, phone, status, is_email_verified,
                failed_login_attempts, two_factor_enabled, preferences,
                created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING *
            "#
        )
        .bind(new_id)
        .bind(request.company_id)
        .bind(request.role_id)
        .bind(request.username)
        .bind(request.email)
        .bind(password_hash)
        .bind(request.first_name)
        .bind(request.last_name)
        .bind(request.phone)
        .bind(UserStatus::Pending)
        .bind(false)
        .bind(0)
        .bind(false)
        .bind(serde_json::json!({}))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool).await?;

        Ok(user)
    }

   
pub async fn find_by_id_sqlite(pool: &SqlitePool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx
            ::query_as::<Sqlite, User>(
                r#"SELECT * FROM users WHERE id = ? AND deleted_at IS NULL"#
            )
            .bind(id)
            .fetch_optional(pool).await?;
        Ok(user)
    }

        pub async fn find_by_id_pg(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx
            ::query_as::<Postgres, User>(
                r#"SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL"#
            )
            .bind(id)
            .fetch_optional(pool).await?;
        Ok(user)
    }
    pub async fn find_by_username_sqlite(
        pool: &SqlitePool,
        username: &str
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx
            ::query_as::<Sqlite, User>(
                r#"SELECT * FROM users WHERE username = ? AND deleted_at IS NULL"#
            )
            .bind(username)
            .fetch_optional(pool).await?;
        Ok(user)
    }

        pub async fn find_by_username_pg(
        pool: &PgPool,
        username: &str
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx
            ::query_as::<Postgres, User>(
                r#"SELECT * FROM users WHERE username = $1 AND deleted_at IS NULL"#
            )
            .bind(username)
            .fetch_optional(pool).await?;
        Ok(user)
    }
    pub async fn find_by_email_pg(
        pool: &PgPool,
        email: &str
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx
            ::query_as::<Postgres, User>(
                r#"SELECT * FROM users WHERE email = $1 AND deleted_at IS NULL"#
            )
            .bind(email)
            .fetch_optional(pool).await?;
        Ok(user)
    }

        pub async fn find_by_email_sqlite(
        pool: &SqlitePool,
        email: &str
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx
            ::query_as::<Sqlite, User>(
                r#"SELECT * FROM users WHERE email = ? AND deleted_at IS NULL"#
            )
            .bind(email)
            .fetch_optional(pool).await?;
        Ok(user)
    }

    pub async fn list_by_company_pg(
        pool: &PgPool,
        company_id: Uuid,
        limit: i64,
        offset: i64
    ) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx
            ::query_as::<Postgres, User>(
                r#"
            SELECT * FROM users
            WHERE company_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
            )
            .bind(company_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool).await?;
        Ok(users)
    }

        pub async fn list_by_company_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        limit: i64,
        offset: i64
    ) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx
            ::query_as::<Sqlite, User>(
                r#"
            SELECT * FROM users
            WHERE company_id = ? AND deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#
            )
            .bind(company_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool).await?;
        Ok(users)
    }

    
    pub async fn update_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        request: UpdateUserRequest,
        updated_by: Uuid
    ) -> Result<User, sqlx::Error> {
        let mut query = String::from(
            "UPDATE users SET updated_by = ?, updated_at = CURRENT_TIMESTAMP"
        );
        let mut bind_count = 2;
        let mut bindings: Vec<String> = vec![];

        if let Some(email) = &request.email {
            query.push_str(&format!(", email = ${}", bind_count));
            bindings.push(email.clone());
            bind_count += 1;
        }

        if let Some(first_name) = &request.first_name {
            query.push_str(&format!(", first_name = ${}", bind_count));
            bindings.push(first_name.clone());
            bind_count += 1;
        }

        if let Some(last_name) = &request.last_name {
            query.push_str(&format!(", last_name = ${}", bind_count));
            bindings.push(last_name.clone());
            bind_count += 1;
        }

        if let Some(phone) = &request.phone {
            query.push_str(&format!(", phone = ${}", bind_count));
            bindings.push(phone.clone());
            bind_count += 1;
        }

        if let Some(avatar_url) = &request.avatar_url {
            query.push_str(&format!(", avatar_url = ${}", bind_count));
            bindings.push(avatar_url.clone());
            bind_count += 1;
        }

        query.push_str(&format!(" WHERE id = ${} RETURNING *", bind_count));

        let mut query_builder = sqlx::query_as::<Sqlite, User>(&query);
        query_builder = query_builder.bind(updated_by);

        for binding in bindings {
            query_builder = query_builder.bind(binding);
        }

        query_builder = query_builder.bind(id);

        let user = query_builder.fetch_one(pool).await?;
        Ok(user)
    }

        pub async fn update_pg(
        pool: &PgPool,
        id: Uuid,
        request: UpdateUserRequest,
        updated_by: Uuid
    ) -> Result<User, sqlx::Error> {
        let mut query = String::from(
            "UPDATE users SET updated_by = $1, updated_at = CURRENT_TIMESTAMP"
        );
        let mut bind_count = 2;
        let mut bindings: Vec<String> = vec![];

        if let Some(email) = &request.email {
            query.push_str(&format!(", email = ${}", bind_count));
            bindings.push(email.clone());
            bind_count += 1;
        }

        if let Some(first_name) = &request.first_name {
            query.push_str(&format!(", first_name = ${}", bind_count));
            bindings.push(first_name.clone());
            bind_count += 1;
        }

        if let Some(last_name) = &request.last_name {
            query.push_str(&format!(", last_name = ${}", bind_count));
            bindings.push(last_name.clone());
            bind_count += 1;
        }

        if let Some(phone) = &request.phone {
            query.push_str(&format!(", phone = ${}", bind_count));
            bindings.push(phone.clone());
            bind_count += 1;
        }

        if let Some(avatar_url) = &request.avatar_url {
            query.push_str(&format!(", avatar_url = ${}", bind_count));
            bindings.push(avatar_url.clone());
            bind_count += 1;
        }

        query.push_str(&format!(" WHERE id = ${} RETURNING *", bind_count));

        let mut query_builder = sqlx::query_as::<Postgres, User>(&query);
        query_builder = query_builder.bind(updated_by);

        for binding in bindings {
            query_builder = query_builder.bind(binding);
        }

        query_builder = query_builder.bind(id);

        let user = query_builder.fetch_one(pool).await?;
        Ok(user)
    }


    pub async fn delete_pg(pool: &PgPool, id: Uuid, deleted_by: Uuid) -> Result<(), sqlx::Error> {
        sqlx
            ::query::<Postgres>(
                r#"
            UPDATE users
            SET deleted_at = CURRENT_TIMESTAMP, updated_by = $1, updated_at = CURRENT_TIMESTAMP
            WHERE id = $2
            "#
            )
            .bind(deleted_by)
            .bind(id)
            .execute(pool).await?;
        Ok(())
    }
    pub async fn delete_sqlite(pool: &SqlitePool, id: Uuid, deleted_by: Uuid) -> Result<(), sqlx::Error> {
        sqlx
            ::query::<Sqlite>(
                r#"
            UPDATE users
            SET deleted_at = CURRENT_TIMESTAMP, updated_by = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#
            )
            .bind(deleted_by)
            .bind(id)
            .execute(pool).await?;
        Ok(())
    }

    pub fn verify_password(&self, password: &str) -> crate::utils::error::Result<bool> {
        crate::utils::password::verify_password(password, &self.password_hash)
    }


    pub async fn update_password_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        new_password: &str,
        updated_by: Uuid
    ) -> Result<(), sqlx::Error> {
        let password_hash = crate::utils::password
            ::hash_password(new_password)
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        sqlx
            ::query::<Sqlite>(
                r#"
            UPDATE users
            SET password_hash = ?, updated_by = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#
            )
            .bind(password_hash)
            .bind(updated_by)
            .bind(id)
            .execute(pool).await?;
        Ok(())
    }
        pub async fn update_password_pg(
        pool: &PgPool,
        id: Uuid,
        new_password: &str,
        updated_by: Uuid
    ) -> Result<(), sqlx::Error> {
        let password_hash = crate::utils::password
            ::hash_password(new_password)
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        sqlx
            ::query::<Postgres>(
                r#"
            UPDATE users
            SET password_hash = $1, updated_by = $2, updated_at = CURRENT_TIMESTAMP
            WHERE id = $3
            "#
            )
            .bind(password_hash)
            .bind(updated_by)
            .bind(id)
            .execute(pool).await?;
        Ok(())
    }

    pub async fn update_last_login_pg(
        pool: &PgPool,
        id: Uuid,
        ip_address: Option<String>
    ) -> Result<(), sqlx::Error> {
        sqlx
            ::query::<Postgres>(
                r#"
            UPDATE users
            SET last_login_at = CURRENT_TIMESTAMP,
                last_login_ip = $1,
                failed_login_attempts = 0
            WHERE id = $2
            "#
            )
            .bind(ip_address)
            .bind(id)
            .execute(pool).await?;
        Ok(())
    }
    pub async fn update_last_login_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        ip_address: Option<String>
    ) -> Result<(), sqlx::Error> {
        sqlx
            ::query::<Sqlite>(
                r#"
            UPDATE users
            SET last_login_at = CURRENT_TIMESTAMP,
                last_login_ip = ?,
                failed_login_attempts = 0
            WHERE id = ?
            "#
            )
            .bind(ip_address)
            .bind(id)
            .execute(pool).await?;
        Ok(())
    }
    pub async fn increment_failed_login_pg(
        pool: &PgPool,
        id: Uuid,
        failed_login_attempts:i32,
        _lockout_duration_minutes: i32
    ) -> Result<(), sqlx::Error> {
        sqlx
            ::query::<Postgres>(
                r#"
            UPDATE users
            SET failed_login_attempts = failed_login_attempts + 1
            WHERE id = $1
            "#
            )
            .bind(id)
            .execute(pool).await?;
        Ok(())
    }

        pub async fn increment_failed_login_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        failed_login_attempts:i32,
        _lockout_duration_minutes: i32
    ) -> Result<(), sqlx::Error> {
        sqlx
            ::query::<Sqlite>(
                r#"
            UPDATE users
            SET failed_login_attempts = failed_login_attempts + 1
            WHERE id = ?
            "#
            )
            .bind(id)
            .execute(pool).await?;
        Ok(())
    }
    pub fn is_locked(&self) -> bool {
        if let Some(locked_until) = self.locked_until { locked_until > Utc::now() } else { false }
    }
}

// ===== ROLE DATABASE OPERATIONS =====
impl Role {
    pub async fn create_sqlite(
        pool: &SqlitePool,
        request: CreateRoleRequest,
        created_by: Uuid
    ) -> Result<Role, sqlx::Error> {
        let role = sqlx::query_as::<Sqlite, Role>(
                r#"
            INSERT INTO roles (
                company_id, name, description, role_type,
                permissions, is_system, is_active, created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
            )
            .bind(request.company_id)
            .bind(request.name)
            .bind(request.description)
            .bind(request.role_type)
            .bind(request.permissions)
            .bind(request.is_system.unwrap_or(false))
            .bind(true)
            .bind(created_by)
            .bind(created_by)
            .fetch_one(pool).await?;
        Ok(role)
    }
    pub async fn create_pg(
        pool: &PgPool,
        request: CreateRoleRequest,
        created_by: Uuid
    ) -> Result<Role, sqlx::Error> {
        let role = sqlx::query_as::<Postgres, Role>(
                r#"
            INSERT INTO roles (
                company_id, name, description, role_type,
                permissions, is_system, is_active, created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
            )
            .bind(request.company_id)
            .bind(request.name)
            .bind(request.description)
            .bind(request.role_type)
            .bind(request.permissions)
            .bind(request.is_system.unwrap_or(false))
            .bind(true)
            .bind(created_by)
            .bind(created_by)
            .fetch_one(pool).await?;
        Ok(role)
    }

    pub async fn find_by_company_default_pg(pool: &PgPool, company_id: Uuid) -> Result<Option<Role>, sqlx::Error> {
        let role = sqlx::query_as::<Postgres, Role>(
            "SELECT * FROM roles WHERE company_id = $1 AND is_system = true AND is_active = true LIMIT 1"
        )
        .bind(company_id)
        .fetch_optional(pool).await?;
        Ok(role)
    }

    pub async fn find_by_company_default_sqlite(pool: &SqlitePool, company_id: Uuid) -> Result<Option<Role>, sqlx::Error> {
        let role = sqlx::query_as::<Sqlite, Role>(
            "SELECT * FROM roles WHERE company_id = ? AND is_system = true AND is_active = true LIMIT 1"
        )
        .bind(company_id)
        .fetch_optional(pool).await?;
        Ok(role)
    }

    // pub async fn find_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Role>, sqlx::Error> {
    //     let role = sqlx::query_as::<Sqlite, Role>(
    //         r#"SELECT * FROM roles WHERE id = $1"#
    //     )
    //     .bind(id)
    //     .fetch_optional(pool)
    //     .await?;
    //     Ok(role)
    // }

    pub async fn find_by_id_pg(pool: &PgPool, id: Uuid) -> Result<Option<Role>, sqlx::Error> {
        let role = sqlx::query_as::<Postgres, Role>(r#"SELECT * FROM roles WHERE id = $1"#)
            .bind(id)
            .fetch_optional(pool).await?;
        Ok(role)
    }

    pub async fn find_by_id_sqlite(
        pool: &SqlitePool,
        id: Uuid
    ) -> Result<Option<Role>, sqlx::Error> {
        let role = sqlx::query_as::<Sqlite, Role>(r#"SELECT * FROM roles WHERE id = ?"#)
            .bind(id)
            .fetch_optional(pool).await?;
        Ok(role)
    }

    pub async fn list_by_company_sqlite(
        pool: &SqlitePool,
        company_id: Uuid
    ) -> Result<Vec<Role>, sqlx::Error> {
        let roles = sqlx::query_as::<Sqlite, Role>(
                r#"
            SELECT * FROM roles
            WHERE company_id = ? AND is_active = true
            ORDER BY name
            "#
            )
            .bind(company_id)
            .fetch_all(pool).await?;
        Ok(roles)
    }
    pub async fn list_by_company_pg(
        pool: &PgPool,
        company_id: Uuid
    ) -> Result<Vec<Role>, sqlx::Error> {
        let roles = sqlx::query_as::<Postgres, Role>(
                r#"SELECT * FROM roles WHERE company_id = $1 AND is_active = true ORDER BY name"#
            )
            .bind(company_id)
            .fetch_all(pool).await?;
        Ok(roles)
    }

pub async fn update_pg(
    pool: &PgPool,
    id: Uuid,
    request: UpdateRoleRequest,
    updated_by: Uuid,
) -> Result<Role, sqlx::Error> {
    let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> = 
        sqlx::QueryBuilder::new("UPDATE roles SET ");

    // Standard tracking fields
    query_builder.push("updated_by = ").push_bind(updated_by);
    query_builder.push(", updated_at = CURRENT_TIMESTAMP");

    // Dynamic fields
    if let Some(name) = request.name {
        query_builder.push(", name = ").push_bind(name);
    }
    if let Some(description) = request.description {
        query_builder.push(", description = ").push_bind(description);
    }
    if let Some(permissions) = request.permissions {
        query_builder.push(", permissions = ").push_bind(permissions);
    }
    if let Some(is_active) = request.is_active {
        query_builder.push(", is_active = ").push_bind(is_active);
    }

    // Filter and Return
    query_builder.push(" WHERE id = ").push_bind(id);
    query_builder.push(" RETURNING *");

    let query = query_builder.build_query_as::<Role>();
    query.fetch_one(pool).await
}
pub async fn update_sqlite(
    pool: &SqlitePool,
    id: Uuid,
    request: UpdateRoleRequest,
    updated_by: Uuid,
) -> Result<Role, sqlx::Error> {
    let mut query_builder: sqlx::QueryBuilder<sqlx::Sqlite> = 
        sqlx::QueryBuilder::new("UPDATE roles SET ");

    query_builder.push("updated_by = ").push_bind(updated_by);
    query_builder.push(", updated_at = CURRENT_TIMESTAMP");

    if let Some(name) = request.name {
        query_builder.push(", name = ").push_bind(name);
    }
    if let Some(description) = request.description {
        query_builder.push(", description = ").push_bind(description);
    }
    if let Some(permissions) = request.permissions {
        query_builder.push(", permissions = ").push_bind(permissions);
    }
    if let Some(is_active) = request.is_active {
        query_builder.push(", is_active = ").push_bind(is_active);
    }

    query_builder.push(" WHERE id = ").push_bind(id);
    query_builder.push(" RETURNING *");

    let query = query_builder.build_query_as::<Role>();
    query.fetch_one(pool).await
}
pub async fn delete_sqlite(pool: &SqlitePool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM roles WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
    pub async fn delete_pg(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM roles WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
    pub fn has_permission(&self, resource: &str, action: &str) -> bool {
        if let Some(resource_perms) = self.permissions.get(resource) {
            if let Some(action_allowed) = resource_perms.get(action) {
                return action_allowed.as_bool().unwrap_or(false);
            }
        }
        false
    }
}

// ===== UNIT TESTS =====
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_role_serialization() {
        let role = UserRole::Admin;
        let json = serde_json::to_string(&role).unwrap();
        assert_eq!(json, r#""admin""#);
    }

    #[test]
    fn test_user_status_serialization() {
        let status = UserStatus::Active;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""active""#);
    }

    #[test]
    fn test_password_validation() {
        assert!(validate_password_strength("SecurePass123!").is_ok());
        assert!(validate_password_strength("weak").is_err());
        assert!(validate_password_strength("NoSpecialChar1").is_err());
    }
}
