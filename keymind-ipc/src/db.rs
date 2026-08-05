use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

pub type SqlitePool = Pool<Sqlite>;

/// Resolves default KeyMind database file path based on operating system.
/// macOS: ~/Library/Application Support/keymind/keymind.db
/// Linux/Unix: ~/.local/share/keymind/keymind.db
pub fn get_default_db_path() -> PathBuf {
    let base_dir = if cfg!(target_os = "macos") {
        dirs_next::data_dir().unwrap_or_else(|| PathBuf::from("~/Library/Application Support"))
    } else {
        dirs_next::data_local_dir().unwrap_or_else(|| PathBuf::from("~/.local/share"))
    };

    let keymind_dir = base_dir.join("keymind");
    let _ = fs::create_dir_all(&keymind_dir);
    keymind_dir.join("keymind.db")
}

/// Initializes SQLite database pool and runs embedded migrations.
pub async fn init_db_pool(db_url: Option<&str>) -> Result<Arc<SqlitePool>, sqlx::Error> {
    let connection_options = if let Some(url) = db_url {
        SqliteConnectOptions::from_str(url)?
            .create_if_missing(true)
    } else {
        let db_path = get_default_db_path();
        SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true)
    };

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await?;

    // Run embedded migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(Arc::new(pool))
}
