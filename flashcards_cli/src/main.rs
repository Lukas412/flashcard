use cards_core::Category;
use clap::{Args, Parser, Subcommand};
use std::convert::Into;
use std::fs;
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
    #[arg(short, long)]
    to: Uuid,
}

#[derive(Debug, Args)]
struct RemoveCommand {
    #[arg(short, long)]
    uuid: Uuid,
}

fn main() -> anyhow::Result<()> {
    let category = load_material()?;
    let arguments = Arguments::parse();

    match arguments.command {
        Command::Add(command) => {
            let uuid = command.to;
        }
        Command::Remove(command) => {
            let uuid = command.uuid;
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
    println!("{} ({})", names.join(" > "), category.uuid());
    for category in category.categories() {
        print_sub_category_list(category, names);
    }
}
