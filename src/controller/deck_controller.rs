use crate::{
    controller::cli::DeckCommand,
    repository::{card_repository::CardRepository, deck_repository::DeckRepository},
    view::deck_view::DeckView,
};
use rusqlite::Connection;

pub struct DeckController;

impl DeckController {
    pub fn handle(conn: &Connection, action: &DeckCommand) {
        match action {
            // DeckCommand::Add { title } => {
            //     DeckRepository::create(conn, title, 999).unwrap();
            //     DeckView::render_created(title);
            // }
            DeckCommand::Ls => {
                let decks = DeckRepository::all(conn).unwrap();
                let mut cards = Vec::new();
                for deck in &decks {
                    let card = CardRepository::all_by_deck_title(conn, &deck.title).unwrap();
                    dbg!(&card);
                    cards.push(card);
                }
                DeckView::render_all(&decks, &cards);
            } // DeckCommand::Rm { id } => {
              //     DeckRepository::delete(conn, *id).unwrap();
              //     DeckView::render_deleted(*id);
              // }
              // DeckCommand::Rename { id, new_title } => {
              //     DeckRepository::update_title(conn, *id, new_title).unwrap();
              //     DeckView::render_renamed(*id, new_title);
              // }
        }
    }
}
