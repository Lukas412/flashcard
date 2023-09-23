use cards_core::{Category, Topic};
use std::fs;

fn main() -> anyhow::Result<()> {
    let raw_topic = fs::read_to_string("material/german/process_description.json")?;
    let topic: Topic = serde_json::from_str(raw_topic.as_str())?;

    let mut category = Category::create("Freitext".into());
    category.add_topic(topic);

    let mut german = Category::create("Deutsch".into());
    german.add_category(category);

    let raw = serde_json::to_string_pretty(&german)?;
    fs::write("material.json", raw)?;

    Ok(())
}
