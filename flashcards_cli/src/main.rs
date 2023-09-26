use crate::arguments::{AddElementCommand, RemoveElementCommand};
use cards_core::{Card, Category, FlashcardStore, Topic};
use clap::{Args, Parser, Subcommand};
use serde_json::json;
use std::convert::Into;
use std::fmt::Arguments;
use std::fs;
use std::process::Command;

mod arguments;

fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();
    let store = FlashcardStore::load_from_bytes("store")?;

    match arguments.command {
        Command::Add(command) => {
            let uuid = command.to;
            println!("{} ({:?})", uuid, command.element);
            match command.element {
                AddElementCommand::Category { name } => {}
                AddElementCommand::Topic { name } => {}
                AddElementCommand::Card { question, answer } => {}
            }
        }
        Command::Remove(command) => {
            let uuid = command.uuid;
            match command.element {
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
        Command::List => print_categories(&store),
    }

    Ok(())
}

fn print_categories(store: &FlashcardStore) {
    
}

fn print_category(category: &Category) {
    let mut names = vec![];
    print_sub_category_list(category, &mut names);
}

fn print_topic(topic: &Topic) {}

fn print_card(card: &Card) {}

fn print_sub_category_list<'a>(category: &'a Category, names: &mut Vec<&'a str>) {
    names.push(category.name());
    let complete_name = names.join(" > ");
    println!("{} ({})", complete_name, category.uuid());
    for topic in category.topics() {
        println!("{} > {} ({})", complete_name, topic.name(), topic.uuid());
    }
    for category in category.categories() {
        print_sub_category_list(category, names);
    }
}
