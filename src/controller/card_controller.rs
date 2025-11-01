use crate::{
    cli::CardCommand, model::card::Card, repository::card_repository::CardRepository,
    view::card_view::CardView,
};
use log::debug;
use rusqlite::Connection;

pub struct CardController;

impl CardController {
    pub fn handle(conn: &Connection, action: &CardCommand) {
        match action {
            CardCommand::Add {
                deck_title,
                card_title,
                description,
            } => {
                CardRepository::create(conn, deck_title, card_title, description.as_deref())
                    .unwrap();
                CardView::render_created(card_title, deck_title);
            }
            CardCommand::Ls { title } => {
                let mut cards: Vec<Card> = Vec::new();
                if let Some(title) = title {
                    debug!("{}", &title);
                    cards = CardRepository::all_by_deck_title(conn, title.clone()).unwrap();
                } else {
                    cards = CardRepository::all(conn).unwrap();
                }
                CardView::render_all(&cards);
            }
            CardCommand::Done { id } => {
                CardRepository::mark_done(conn, *id).unwrap();
                CardView::render_done(*id);
            }
            CardCommand::Move { id, target_deck_id } => {
                CardRepository::move_to_deck(conn, *id, *target_deck_id).unwrap();
                CardView::render_moved(*id, *target_deck_id);
            }
            CardCommand::Rm { id } => {
                CardRepository::delete(conn, *id).unwrap();
                CardView::render_deleted(*id);
            }
        }
    }
}
