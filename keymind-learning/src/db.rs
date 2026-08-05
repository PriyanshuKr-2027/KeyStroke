use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnedPhrase {
    pub id: String,
    pub phrase: String,
    pub frequency: i32,
    pub is_pinned: bool,
}

pub async fn init_learning_tables(db: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS phrase_candidates (
            phrase TEXT PRIMARY KEY,
            frequency INTEGER DEFAULT 1,
            first_seen INTEGER DEFAULT (unixepoch()),
            last_seen INTEGER DEFAULT (unixepoch())
        );
        CREATE TABLE IF NOT EXISTS learned_memory (
            id TEXT PRIMARY KEY,
            phrase TEXT UNIQUE NOT NULL,
            frequency INTEGER DEFAULT 1,
            is_pinned INTEGER DEFAULT 0
        );
        CREATE TABLE IF NOT EXISTS app_blocklist (
            app_id TEXT PRIMARY KEY
        );
        "#,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn upsert_candidate(
    db: &Pool<Sqlite>,
    phrase: &str,
) -> Result<Option<LearnedPhrase>, sqlx::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Upsert candidate count
    sqlx::query(
        r#"
        INSERT INTO phrase_candidates (phrase, frequency, first_seen, last_seen)
        VALUES (?1, 1, ?2, ?2)
        ON CONFLICT(phrase) DO UPDATE SET
            frequency = frequency + 1,
            last_seen = ?2;
        "#,
    )
    .bind(phrase)
    .bind(now)
    .execute(db)
    .await?;

    // Check promotion threshold
    let row = sqlx::query_as::<_, (i32, i64)>(
        "SELECT frequency, last_seen FROM phrase_candidates WHERE phrase = ?1",
    )
    .bind(phrase)
    .fetch_optional(db)
    .await?;

    if let Some((freq, last_seen)) = row {
        let seven_days_ago = now - (7 * 86400);
        let should_promote = (freq >= 3 && last_seen >= seven_days_ago) || freq >= 10;

        if should_promote {
            let id = Uuid::new_v4().to_string();
            sqlx::query(
                r#"
                INSERT INTO learned_memory (id, phrase, frequency, is_pinned)
                VALUES (?1, ?2, ?3, 0)
                ON CONFLICT(phrase) DO UPDATE SET frequency = ?3;
                "#,
            )
            .bind(&id)
            .bind(phrase)
            .bind(freq)
            .execute(db)
            .await?;

            return Ok(Some(LearnedPhrase {
                id,
                phrase: phrase.to_string(),
                frequency: freq,
                is_pinned: false,
            }));
        }
    }

    Ok(None)
}

pub async fn prune_old_candidates(db: &Pool<Sqlite>) -> Result<u64, sqlx::Error> {
    let thirty_days_ago = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
        - (30 * 86400);

    let res = sqlx::query(
        "DELETE FROM phrase_candidates WHERE last_seen < ?1 AND frequency < 2",
    )
    .bind(thirty_days_ago)
    .execute(db)
    .await?;

    Ok(res.rows_affected())
}

pub async fn get_learned_phrases(db: &Pool<Sqlite>) -> Result<Vec<LearnedPhrase>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String, String, i32, i32)>(
        "SELECT id, phrase, frequency, is_pinned FROM learned_memory ORDER BY is_pinned DESC, frequency DESC",
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(id, phrase, frequency, is_pinned)| LearnedPhrase {
            id,
            phrase,
            frequency,
            is_pinned: is_pinned != 0,
        })
        .collect())
}

pub async fn pin_phrase(id: &str, db: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE learned_memory SET is_pinned = 1 WHERE id = ?1")
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn delete_phrase(id: &str, db: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM learned_memory WHERE id = ?1")
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn ignore_phrase(id: &str, db: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    delete_phrase(id, db).await
}

pub async fn add_app_to_blocklist(app_id: &str, db: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT OR IGNORE INTO app_blocklist (app_id) VALUES (?1)")
        .bind(app_id)
        .execute(db)
        .await?;
    Ok(())
}
