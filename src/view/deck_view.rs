use crate::model::card::Card;
use crate::model::deck::Deck;
use colored::*; // nếu bạn muốn in màu (cargo add colored)

pub struct DeckView;

impl DeckView {
    pub fn render_all(decks: &[Deck], cards: &[Vec<Card>]) {
        for (i, deck) in decks.iter().enumerate() {
            print!(
                "[{}] {} ({} cards)",
                deck.id.to_string().yellow(),
                deck.title.cyan(),
                deck.num_card.to_string(),
            );

            if let Some(card_list) = cards.get(i)
                && !card_list.is_empty()
            {
                let preview_cards: Vec<String> =
                    card_list.iter().take(3).map(|c| c.title.clone()).collect();

                let joined = preview_cards.join(", ");
                print!(" - {}", joined);
            }

            println!();
        }
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
