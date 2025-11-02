use std::result;

use crate::{model::card::Card, repository::deck_repository::DeckRepository};
use chrono::Local;
use log::debug;
use rusqlite::{Connection, Result, params};

pub struct CardRepository;

impl CardRepository {
    pub fn create(
        conn: &Connection,
        deck_title: &str,
        card_title: &str,
        description: Option<&str>,
    ) -> Result<()> {
        let mut stmt = conn.prepare("SELECT id FROM decks WHERE title = ?1")?;
        let deck_id: i32 = stmt.query_row(params![deck_title], |row| row.get(0))?;

        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        conn.execute(
            "INSERT INTO cards (title, description, deck_id, deck_title, position, created_at)
             VALUES (
                 ?1, ?2, ?3, ?4,
                 COALESCE((SELECT MAX(position)+1 FROM cards WHERE deck_id=?3), 1),
                 ?5
             )",
            params![card_title, description, deck_id, deck_title, now],
        )?;

        Ok(())
    }

    pub fn all(conn: &Connection) -> Result<Vec<Card>> {
        let mut stmt = conn.prepare(
            "SELECT id, title, description, deck_id, deck_title, position, done, created_at
            FROM cards",
        )?;
        let rows = stmt
            .query_map([], |row| {
                Ok(Card {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    deck_id: row.get(3)?,
                    deck_title: row.get(4)?,
                    position: row.get(5)?,
                    done: row.get::<_, i32>(6)? == 1,
                    created_at: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn all_by_deck_title(conn: &Connection, deck_title: String) -> Result<Vec<Card>> {
        let mut stmt = conn.prepare(
            "SELECT id, title, description, deck_id, deck_title, position, done, created_at
             FROM cards WHERE deck_title = ?1 ORDER BY position ASC",
        )?;
        let rows = stmt
            .query_map(params![deck_title], |row| {
                Ok(Card {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    deck_id: row.get(3)?,
                    deck_title: row.get(4)?,
                    position: row.get(5)?,
                    done: row.get::<_, i32>(6)? == 1,
                    created_at: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn mark_done(conn: &Connection, id: i32) -> Result<()> {
        let num_row_affected =
            conn.execute("UPDATE cards SET done = 1 WHERE id = ?1", params![id])?;
        if num_row_affected == 0 {
            Err(rusqlite::Error::QueryReturnedNoRows)
        } else {
            Ok(())
        }
    }

    pub fn get_by_id(conn: &Connection, id: i32) -> Result<(Card)> {
        let card = conn.query_row(
            "SELECT
                id,
                title,
                description,
                deck_id,
                deck_title,
                position,
                done,
                created_at
            FROM cards
            WHERE id = ?1
            ",
            params![id],
            |row| {
                Ok(Card {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    deck_id: row.get(3)?,
                    deck_title: row.get(4)?,
                    position: row.get(5)?,
                    done: row.get(6)?,
                    created_at: row.get(7)?,
                })
            },
        );
        card
    }

    pub fn move_to_deck(conn: &Connection, id: i32, new_deck_title: &String) -> Result<()> {
        let card = CardRepository::get_by_id(conn, id).unwrap();
        let new_deck = DeckRepository::get_by_title(conn, new_deck_title).unwrap();
        let current_deck = DeckRepository::get_by_id(conn, card.deck_id).unwrap();
        DeckRepository::add_card(conn, new_deck_title)?;
        DeckRepository::rm_card(conn, &current_deck.title)?;
        conn.execute(
            "UPDATE cards SET deck_id = ?1, deck_title = ?2 WHERE id = ?3",
            params![new_deck.id, new_deck.title, id],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i32) -> Result<()> {
        conn.execute("DELETE FROM cards WHERE id = ?1", params![id])?;
        Ok(())
    }
}
