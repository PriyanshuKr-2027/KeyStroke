use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

pub type SqlitePool = Pool<Sqlite>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VarType {
    Static,
    Dynamic,
    Ai,
}

impl VarType {
    pub fn as_str(&self) -> &'static str {
        match self {
            VarType::Static => "static",
            VarType::Dynamic => "dynamic",
            VarType::Ai => "ai",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "dynamic" => VarType::Dynamic,
            "ai" => VarType::Ai,
            _ => VarType::Static,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub key: String,
    pub var_type: VarType,
    pub value: Option<String>,
    pub ai_prompt: Option<String>,
    pub use_count: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct DbHandler {
    pool: Arc<SqlitePool>,
}

impl DbHandler {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn init_db(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS variables (
                key       TEXT PRIMARY KEY,
                var_type  TEXT NOT NULL CHECK(var_type IN ('static','dynamic','ai')),
                value     TEXT,
                ai_prompt TEXT,
                use_count INTEGER DEFAULT 0,
                created_at INTEGER DEFAULT (unixepoch()),
                updated_at INTEGER DEFAULT (unixepoch())
            );",
        )
        .execute(self.pool.as_ref())
        .await?;

        Ok(())
    }

    pub async fn upsert(&self, v: Variable) -> Result<(), sqlx::Error> {
        let key_clean = v.key.trim_start_matches('/').to_lowercase();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        sqlx::query(
            "INSERT INTO variables (key, var_type, value, ai_prompt, use_count, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(key) DO UPDATE SET
             var_type = excluded.var_type,
             value = excluded.value,
             ai_prompt = excluded.ai_prompt,
             use_count = excluded.use_count,
             updated_at = excluded.updated_at",
        )
        .bind(&key_clean)
        .bind(v.var_type.as_str())
        .bind(v.value)
        .bind(v.ai_prompt)
        .bind(v.use_count)
        .bind(now)
        .bind(now)
        .execute(self.pool.as_ref())
        .await?;

        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<(), sqlx::Error> {
        let key_clean = key.trim_start_matches('/').to_lowercase();
        sqlx::query("DELETE FROM variables WHERE key = ?")
            .bind(&key_clean)
            .execute(self.pool.as_ref())
            .await?;

        Ok(())
    }

    pub async fn list_all(&self) -> Result<Vec<Variable>, sqlx::Error> {
        let rows: Vec<(String, String, Option<String>, Option<String>, i64, i64, i64)> =
            sqlx::query_as(
                "SELECT key, var_type, value, ai_prompt, use_count, created_at, updated_at FROM variables",
            )
            .fetch_all(self.pool.as_ref())
            .await?;

        Ok(rows
            .into_iter()
            .map(
                |(key, var_type, value, ai_prompt, use_count, created_at, updated_at)| Variable {
                    key,
                    var_type: VarType::from_str(&var_type),
                    value,
                    ai_prompt,
                    use_count,
                    created_at,
                    updated_at,
                },
            )
            .collect())
    }

    pub async fn get(&self, key: &str) -> Result<Option<Variable>, sqlx::Error> {
        let key_clean = key.trim_start_matches('/').to_lowercase();
        let row: Option<(String, String, Option<String>, Option<String>, i64, i64, i64)> =
            sqlx::query_as(
                "SELECT key, var_type, value, ai_prompt, use_count, created_at, updated_at FROM variables WHERE key = ?",
            )
            .bind(&key_clean)
            .fetch_optional(self.pool.as_ref())
            .await?;

        Ok(row.map(
            |(key, var_type, value, ai_prompt, use_count, created_at, updated_at)| Variable {
                key,
                var_type: VarType::from_str(&var_type),
                value,
                ai_prompt,
                use_count,
                created_at,
                updated_at,
            },
        ))
    }

    pub async fn increment_use_count(&self, key: &str) -> Result<(), sqlx::Error> {
        let key_clean = key.trim_start_matches('/').to_lowercase();
        sqlx::query("UPDATE variables SET use_count = use_count + 1 WHERE key = ?")
            .bind(&key_clean)
            .execute(self.pool.as_ref())
            .await?;

        Ok(())
    }
}
