mod cli;
mod controller;
mod model;
mod repository;
mod util;
mod view;

use clap::Parser;
use cli::{Cli, Commands};
use controller::{card_controller::CardController, deck_controller::DeckController};
use util::db::init_db;

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    let conn = init_db().expect("Không thể mở hoặc tạo kanbuk.db");

    match &cli.command {
        Commands::Deck { action } => DeckController::handle(&conn, action),
        Commands::Card { action } => CardController::handle(&conn, action),
    }
}
