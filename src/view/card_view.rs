use crate::model::card::Card;
use colored::*;

pub struct CardView;

impl CardView {
    pub fn render_all(cards: &[Card]) {
        for card in cards {
            let status = if card.done { "✅" } else { "❌" };
            println!(
                "[{}] {} {}",
                card.id.to_string().yellow(),
                status,
                card.title.cyan()
            );
        }
        println!();
    }

    pub fn render_created(card_title: &str, deck_title: &str) {
        println!("add {} into {}", card_title.green(), deck_title.green());
    }

    pub fn render_done(id: i32) {
        println!("card #{} is completed!", id);
    }

    pub fn render_moved(id: i32, target_list_id: i32) {
        println!("card #{} is moved to #{}", id, target_list_id);
    }

    pub fn render_deleted(id: i32) {
        println!("delete card #{}", id);
    }
}
