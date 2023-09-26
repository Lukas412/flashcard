use crate::arguments::{
    AddElementCommand, FlashCardArguments, FlashCardCommand, RemoveElementCommand,
};
use cards_core::FlashCardStore;
use clap::Parser;
use serde_json::json;

mod arguments;

fn main() -> anyhow::Result<()> {
    let arguments = FlashCardArguments::parse();
    let mut store = FlashCardStore::load_from_bytes("store")?;

    match arguments.command() {
        FlashCardCommand::Add(command) => {
            let uuid = command.to();
            println!("{} ({:?})", uuid, command.element());
            match command.element() {
                AddElementCommand::Category { name } => {}
                AddElementCommand::Topic { name } => {}
                AddElementCommand::Card { question, answer } => {}
            }
        }
        FlashCardCommand::Remove(command) => {
            let uuid = command.uuid();
            match command.element() {
                RemoveElementCommand::Category => {
                    if let Some(removed) = store.remove_category(&uuid) {
                        println!("removed:\n{}", json!(removed));
                    }
                }
                RemoveElementCommand::Topic => {
                    if let Some(topic) = store.remove_topic(&uuid) {
                        println!("removed:\n{}", json!(topic));
                    }
                }
                RemoveElementCommand::Card => {
                    if let Some(card) = store.remove_card(&uuid) {
                        println!("removed:\n{}", json!(card))
                    }
                }
            }
        }
        FlashCardCommand::List => print_categories(&store),
    }

    Ok(())
}

fn print_categories(store: &FlashCardStore) {
    for (uuid, category) in store.categories() {
        println!("{} ({})", category.name(), uuid);
        for (uuid, topic) in store.topics_for(uuid) {
            println!("{} > {} ({})", category.name(), topic.name(), uuid);
        }
    }
}
