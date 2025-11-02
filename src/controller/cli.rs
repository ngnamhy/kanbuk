use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kanbuk")]
#[command(author = "ngnamhy <hy.nguyen@endava.com>")]
#[command(version = "0.1")]
#[command(about = "Kanbuk CLI - Task management in Kanban style", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Deck {
        #[command(subcommand)]
        action: DeckCommand,
    },

    Card {
        #[command(subcommand)]
        action: CardCommand,
    },

    Db {
        #[command(subcommand)]
        action: DbCommand,
    },
}

#[derive(Subcommand)]
pub enum DbCommand {
    Init,
}

#[derive(Subcommand)]
pub enum DeckCommand {
    // Add { title: String },
    Ls,
    // Rm { id: i32 },

    // Rename { id: i32, new_title: String },
}

#[derive(Subcommand)]
pub enum CardCommand {
    Add {
        deck_title: String,
        card_title: String,
        description: Option<String>,
    },

    Ls {
        title: Option<String>,
    },

    Done {
        id: i32,
    },

    Mv {
        id: i32,
        target_deck_title: String,
    },

    /// Xóa card
    Rm {
        id: i32,
    },
}
