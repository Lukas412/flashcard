use cards_core::Topic;
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let raw_topic = read_to_string("material/german/process_description.json")?;
    let topic: Topic = serde_json::from_str(raw_topic.as_str())?;

    dbg!(topic);

    Ok(())
}
