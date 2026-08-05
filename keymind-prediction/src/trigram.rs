use sqlx::{Pool, Sqlite};
use tracing::info;

pub async fn init_trigram_table(db: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trigrams (
            w1 TEXT NOT NULL,
            w2 TEXT NOT NULL,
            w3 TEXT NOT NULL,
            count INTEGER DEFAULT 1,
            PRIMARY KEY (w1, w2, w3)
        );
        CREATE INDEX IF NOT EXISTS idx_trigrams_w1_w2 ON trigrams(w1, w2);
        "#,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn load_bundled_trigrams(
    db: &Pool<Sqlite>,
    tsv_content: &str,
) -> Result<usize, sqlx::Error> {
    let mut count = 0;
    let mut tx = db.begin().await?;

    for line in tsv_content.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 4 {
            let w1 = parts[0].trim().to_lowercase();
            let w2 = parts[1].trim().to_lowercase();
            let w3 = parts[2].trim().to_lowercase();
            let cnt: i64 = parts[3].trim().parse().unwrap_or(1);

            sqlx::query(
                r#"
                INSERT OR IGNORE INTO trigrams (w1, w2, w3, count)
                VALUES (?1, ?2, ?3, ?4);
                "#,
            )
            .bind(&w1)
            .bind(&w2)
            .bind(&w3)
            .bind(cnt)
            .execute(&mut *tx)
            .await?;

            count += 1;
        }
    }

    tx.commit().await?;
    info!("Preloaded {} trigrams into SQLite", count);
    Ok(count)
}

pub async fn query_trigrams(
    db: &Pool<Sqlite>,
    w1: &str,
    w2: &str,
) -> Result<(Vec<String>, f32), sqlx::Error> {
    let w1_clean = w1.trim().to_lowercase();
    let w2_clean = w2.trim().to_lowercase();

    let rows = sqlx::query_as::<_, (String, i64)>(
        r#"
        SELECT w3, count FROM trigrams
        WHERE w1 = ?1 AND w2 = ?2
        ORDER BY count DESC
        LIMIT 3;
        "#,
    )
    .bind(&w1_clean)
    .bind(&w2_clean)
    .fetch_all(db)
    .await?;

    if rows.is_empty() {
        return Ok((Vec::new(), 0.0));
    }

    let total: i64 = rows.iter().map(|(_, c)| c).sum();
    let top_count = rows[0].1;
    let confidence = if total > 0 {
        (top_count as f32) / (total as f32)
    } else {
        0.0
    };

    let suggestions: Vec<String> = rows.into_iter().map(|(w3, _)| w3).collect();
    Ok((suggestions, confidence))
}

pub async fn update_trigram(
    db: &Pool<Sqlite>,
    w1: &str,
    w2: &str,
    w3: &str,
) -> Result<(), sqlx::Error> {
    let w1_clean = w1.trim().to_lowercase();
    let w2_clean = w2.trim().to_lowercase();
    let w3_clean = w3.trim().to_lowercase();

    if w1_clean.is_empty() || w2_clean.is_empty() || w3_clean.is_empty() {
        return Ok(());
    }

    sqlx::query(
        r#"
        INSERT INTO trigrams (w1, w2, w3, count)
        VALUES (?1, ?2, ?3, 1)
        ON CONFLICT(w1, w2, w3) DO UPDATE SET count = count + 1;
        "#,
    )
    .bind(w1_clean)
    .bind(w2_clean)
    .bind(w3_clean)
    .execute(db)
    .await?;

    Ok(())
}
