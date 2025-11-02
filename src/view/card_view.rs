use crate::model::card::Card;
use colored::*;

pub struct CardView;

impl CardView {
    pub fn render_all(cards: &[Card]) {
        for card in cards {
            println!(
                "[{}] {} - {}",
                card.id.to_string().yellow(),
                card.title.cyan(),
                card.deck_title,
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

    pub fn render_moved(id: i32, target_list_title: &str) {
        println!("card #{} is moved to #{}", id, target_list_title);
    }

    pub fn render_deleted(id: i32) {
        println!("delete card #{}", id);
    }

    pub fn render_error(err: &str) {
        println!("err: {}", err);
    }
}
