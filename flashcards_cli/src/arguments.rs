use clap::{Args, Parser, Subcommand};
use uuid::Uuid;

/// CLI application to add, remove and list flashcards.
#[derive(Parser)]
#[command(author, version, about)]
pub(crate) struct FlashCardArguments {
    #[clap(subcommand)]
    command: FlashCardCommand,
}

impl FlashCardArguments {
    pub(crate) fn command(&self) -> &FlashCardCommand {
        &self.command
    }
}

#[derive(Debug, Subcommand)]
pub(crate) enum FlashCardCommand {
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

impl AddCommand {
    pub(crate) fn element(&self) -> &AddElementCommand {
        &self.element
    }

    pub(crate) fn to(&self) -> &Uuid {
        &self.to
    }
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

impl RemoveCommand {
    pub(crate) fn element(&self) -> &RemoveElementCommand {
        &self.element
    }

    pub(crate) fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}

#[derive(Debug, Subcommand)]
pub(crate) enum RemoveElementCommand {
    Category,
    Topic,
    Card,
}
