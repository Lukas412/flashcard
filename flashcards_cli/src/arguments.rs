use clap::{Args, Parser, Subcommand};
use uuid::Uuid;

/// CLI application to add, remove and list flashcards.
#[derive(Parser)]
#[command(author, version, about)]
pub(crate) struct Arguments {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    Add(AddCommand),
    Remove(RemoveCommand),
    List,
}

#[derive(Debug, Args)]
pub(crate) struct AddCommand {
    #[clap(subcommand)]
    element: AddElementCommand,
    #[arg(short, long)]
    to: Uuid,
}

#[derive(Debug, Subcommand)]
pub(crate) enum AddElementCommand {
    Category {
        #[arg(short, long)]
        name: String,
    },
    Topic {
        #[arg(short, long)]
        name: String,
    },
    Card {
        #[arg(short, long)]
        question: String,
        #[arg(short, long)]
        answer: String,
    },
}

#[derive(Debug, Args)]
pub(crate) struct RemoveCommand {
    #[clap(subcommand)]
    element: RemoveElementCommand,
    #[arg(short, long)]
    uuid: Uuid,
}

#[derive(Debug, Subcommand)]
pub(crate) enum RemoveElementCommand {
    Category,
    Topic,
    Card,
}
