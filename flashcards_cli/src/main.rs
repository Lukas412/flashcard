use cards_core::Category;
use clap::{Args, Parser, Subcommand};
use serde_json::json;
use std::convert::Into;
use std::fs;
use std::iter::once;
use uuid::Uuid;

/// CLI application to add, remove and list flashcards.
#[derive(Parser)]
#[command(author, version, about)]
struct Arguments {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Add(AddCommand),
    Remove(RemoveCommand),
    List,
}

#[derive(Debug, Args)]
struct AddCommand {
    #[clap(subcommand)]
    element: AddElementCommand,
    #[arg(short, long)]
    to: Uuid,
}

#[derive(Debug, Subcommand)]
enum AddElementCommand {
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
struct RemoveCommand {
    #[clap(subcommand)]
    element: RemoveElementCommand,
    #[arg(short, long)]
    uuid: Uuid,
}

#[derive(Debug, Subcommand)]
enum RemoveElementCommand {
    Category,
    Topic,
    Card,
}

fn main() -> anyhow::Result<()> {
    let mut category = load_material()?;
    let arguments = Arguments::parse();

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
                    if let Some(removed) = category.remove_category(&uuid) {
                        println!("removed:\n{}", json!(removed));
                    }
                }
                RemoveElementCommand::Topic => {
                    if let Some(topic) = category.remove_topic(&uuid) {
                        println!("removed:\n{}", json!(topic));
                    }
                }
                RemoveElementCommand::Card => {
                    if let Some(card) = category.remove_card(&uuid) {
                        println!("removed:\n{}", json!(card))
                    }
                }
            }
        }
        Command::List => print_category_list(&category),
    }

    Ok(())
}

fn load_material() -> anyhow::Result<Category> {
    let raw_material = fs::read_to_string("material.json")?;
    serde_json::from_str(&raw_material).map_err(Into::into)
}

fn save_material(category: &Category) -> anyhow::Result<()> {
    let raw_material = serde_json::to_string_pretty(category)?;
    fs::write("material.json", raw_material).map_err(Into::into)
}

fn print_category_list(category: &Category) {
    let mut names = vec![];
    print_sub_category_list(category, &mut names);
}

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
