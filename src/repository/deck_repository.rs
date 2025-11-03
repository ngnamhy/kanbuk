use crate::model::deck::Deck;
use chrono::Local;
use rusqlite::{Connection, Result, params};

pub struct DeckRepository;

impl DeckRepository {
    pub fn create(conn: &Connection, title: &str, position: i32) -> Result<()> {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        conn.execute(
            "INSERT INTO decks (title, position, created_at) VALUES (?1, ?2, ?3)",
            params![title, position, now],
        )?;
        Ok(())
    }

    pub fn get_by_id(conn: &Connection, id: i32) -> Result<(Deck)> {
        conn.query_row(
            "SELECT id, title, position, created_at, num_card FROM decks WHERE id = ?1",
            params![id],
            |row| {
                Ok(Deck {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    position: row.get(2)?,
                    created_at: row.get(3)?,
                    num_card: row.get(4)?,
                })
            },
        )
    }

    pub fn get_by_title(conn: &Connection, title: &str) -> Result<(Deck)> {
        let deck = conn.query_row(
            "SELECT id, title, position, created_at, num_card FROM decks WHERE title = ?1",
            params![title],
            |row| {
                Ok(Deck {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    position: row.get(2)?,
                    created_at: row.get(3)?,
                    num_card: row.get(4)?,
                })
            },
        );
        dbg!(&deck);
        deck
    }

    pub fn all(conn: &Connection) -> Result<Vec<Deck>> {
        let mut stmt = conn.prepare(
            "SELECT id, title, position, created_at, num_card FROM decks ORDER BY position ASC",
        )?;
        let rows = stmt
            .query_map([], |row| {
                Ok(Deck {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    position: row.get(2)?,
                    created_at: row.get(3)?,
                    num_card: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn update_num_card(conn: &Connection, id: i32, num_card: i32) -> Result<()> {
        conn.execute(
            "UPDATE decks SET num_card = ?1 WHERE id = ?2",
            params![num_card, id],
        )?;
        Ok(())
    }

    pub fn update_title(conn: &Connection, id: i32, new_title: &str) -> Result<()> {
        conn.execute(
            "UPDATE decks SET title = ?1 WHERE id = ?2",
            params![new_title, id],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i32) -> Result<()> {
        conn.execute("DELETE FROM decks WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn add_card(conn: &Connection, title: &str) -> Result<()> {
        let deck = DeckRepository::get_by_title(conn, title)?;
        DeckRepository::update_num_card(conn, deck.id, deck.num_card + 1)?;
        Ok(())
    }

    pub fn rm_card(conn: &Connection, title: &str) -> Result<()> {
        let deck = DeckRepository::get_by_title(conn, title)?;
        DeckRepository::update_num_card(conn, deck.id, deck.num_card - 1)?;
        Ok(())
    }
}
