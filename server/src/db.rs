//! SQLite storage. Opening the database and the accounts table actually work; everything a real
//! server would need to query or write beyond that is not implemented.

use rusqlite::Connection;
use std::path::Path;

/// Opens (or creates) the database file and makes sure the base tables exist.
pub fn open(path: &Path) -> rusqlite::Result<Connection> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).ok();
    }
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id TEXT PRIMARY KEY,
            display_name TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

/// Where the database lives by default.
pub fn default_path() -> std::path::PathBuf {
    std::path::PathBuf::from("retroverse-server.sqlite")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opens_an_in_memory_style_file() {
        let path = std::env::temp_dir().join("retroverse-db-test.sqlite");
        let _ = std::fs::remove_file(&path);
        let conn = open(&path).expect("open");
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM accounts", [], |row| row.get(0))
            .expect("query");
        assert_eq!(count, 0);
        let _ = std::fs::remove_file(&path);
    }
}
