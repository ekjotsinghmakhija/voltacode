// ============================================================================
// Copyright (c) 2026 Ekjot Singh
// Contact: ekjotmakhija@gmail.com
// GitHub: https://github.com/ekjotsinghmakhija
//
// Project: Voltacode
// Description: High-performance intelligent coding agent and terminal UI.
// ============================================================================

use rusqlite::{Connection, Result};
use std::path::Path;

pub struct VoltacodeDB {
    conn: Connection,
}

impl VoltacodeDB {
    /// Initializes a new connection to the local SQLite state store.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = VoltacodeDB { conn };
        db.apply_migrations()?;
        Ok(db)
    }

    /// Executes strictly defined structural schema migrations
    fn apply_migrations(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;

            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                status TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS interactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                FOREIGN KEY(session_id) REFERENCES sessions(id) ON DELETE CASCADE
            );
            ",
        )?;
        Ok(())
    }
}
