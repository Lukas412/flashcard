use cards_core::Topic;
use std::fs;

fn main() -> anyhow::Result<()> {
    let raw_topic = fs::read_to_string("material/german/process_description.json")?;
    let mut topic: Topic = serde_json::from_str(raw_topic.as_str())?;
    topic.generate_uuids();

    let raw = serde_json::to_string_pretty(&topic)?;
    fs::write("material/german/process_description.json", raw)?;

    Ok(())
}
