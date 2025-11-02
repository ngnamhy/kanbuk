use crate::model::deck::Deck;
use colored::*; // nếu bạn muốn in màu (cargo add colored)

pub struct DeckView;

impl DeckView {
    pub fn render_all(decks: &[Deck]) {
        for deck in decks {
            println!(
                "[{}] {} ({} cards)",
                deck.id.to_string().yellow(),
                deck.title.cyan(),
                deck.num_card.to_string(),
            );
        }
        println!();
    }

    pub fn render_created(title: &str) {
        println!("created '{}'", title.green());
    }

    pub fn render_deleted(id: i32) {
        println!("deleted #{}", id);
    }

    pub fn render_renamed(id: i32, new_title: &str) {
        println!("rename #{} to '{}'", id, new_title);
    }
}
