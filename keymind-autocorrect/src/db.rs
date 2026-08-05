use sqlx::{Pool, Sqlite};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub type SqlitePool = Pool<Sqlite>;

pub struct DbHandler {
    pub pool: Arc<SqlitePool>,
}

impl DbHandler {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    /// Initialize SQLite database tables.
    pub async fn init_db(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS personal_words (
                word TEXT PRIMARY KEY
            );",
        )
        .execute(self.pool.as_ref())
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS corrections (
                from_word TEXT NOT NULL,
                to_word TEXT NOT NULL,
                count INTEGER NOT NULL DEFAULT 1,
                last_seen INTEGER NOT NULL,
                PRIMARY KEY (from_word, to_word)
            );",
        )
        .execute(self.pool.as_ref())
        .await?;

        Ok(())
    }

    /// Load all personal words into a HashSet.
    pub async fn load_personal_words(&self) -> Result<HashSet<String>, sqlx::Error> {
        let rows: Vec<(String,)> = sqlx::query_as("SELECT word FROM personal_words")
            .fetch_all(self.pool.as_ref())
            .await?;

        Ok(rows.into_iter().map(|(w,)| w.to_lowercase()).collect())
    }

    /// Load user learned corrections where count >= 3 into a HashMap (from_word -> to_word).
    pub async fn load_learned_corrections(&self) -> Result<HashMap<String, String>, sqlx::Error> {
        let rows: Vec<(String, String)> =
            sqlx::query_as("SELECT from_word, to_word FROM corrections WHERE count >= 3")
                .fetch_all(self.pool.as_ref())
                .await?;

        Ok(rows
            .into_iter()
            .map(|(from, to)| (from.to_lowercase(), to))
            .collect())
    }

    /// Insert or ignore word into personal_words table.
    pub async fn insert_personal_word(&self, word: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT OR IGNORE INTO personal_words (word) VALUES (?)")
            .bind(word.to_lowercase())
            .execute(self.pool.as_ref())
            .await?;

        Ok(())
    }

    /// Upsert user correction and return new count.
    pub async fn record_correction(&self, from: &str, to: &str) -> Result<i64, sqlx::Error> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let from_lower = from.to_lowercase();

        sqlx::query(
            "INSERT INTO corrections (from_word, to_word, count, last_seen)
             VALUES (?, ?, 1, ?)
             ON CONFLICT(from_word, to_word) DO UPDATE SET
             count = count + 1,
             last_seen = excluded.last_seen",
        )
        .bind(&from_lower)
        .bind(to)
        .bind(now)
        .execute(self.pool.as_ref())
        .await?;

        let count: (i64,) = sqlx::query_as(
            "SELECT count FROM corrections WHERE from_word = ? AND to_word = ?",
        )
        .bind(&from_lower)
        .bind(to)
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(count.0)
    }
}
