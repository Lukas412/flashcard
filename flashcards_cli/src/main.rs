use cards_core::Topic;
use std::fs;

fn main() -> anyhow::Result<()> {
    let raw_topic = fs::read_to_string("material/german/process_description.json")?;
    let topic: Topic = serde_json::from_str(raw_topic.as_str())?;

    Ok(())
}
