use dirs;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

pub fn init_db() -> Result<Connection> {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let db_dir = home.join(".kanbuk");
    let db_path = db_dir.join("kanbuk.db");

    fs::create_dir_all(&db_dir).expect("cannot create directory ~/.kanbuk");

    let conn = Connection::open(db_path)?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS decks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL UNIQUE,
            position INTEGER DEFAULT 0,
            num_card INTEGER DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS cards (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            deck_id INTEGER NOT NULL,
            deck_title TEXT,
            position INTEGER DEFAULT 0,
            done INTEGER DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (deck_id) REFERENCES decks(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_cards_deck_id ON cards(deck_id)",
        [],
    )?;

    conn.execute(
        "INSERT INTO decks (title, position)
         SELECT 'backlog', 1
         WHERE NOT EXISTS (SELECT 1 FROM decks WHERE title='backlog')",
        [],
    )?;
    conn.execute(
        "INSERT INTO decks (title, position)
         SELECT 'todo', 2
         WHERE NOT EXISTS (SELECT 1 FROM decks WHERE title='todo')",
        [],
    )?;
    conn.execute(
        "INSERT INTO decks (title, position)
         SELECT 'doing', 3
         WHERE NOT EXISTS (SELECT 1 FROM decks WHERE title='doing')",
        [],
    )?;
    conn.execute(
        "INSERT INTO decks (title, position)
         SELECT 'done', 4
         WHERE NOT EXISTS (SELECT 1 FROM decks WHERE title='done')",
        [],
    )?;

    Ok(conn)
}
