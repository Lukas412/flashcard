use crate::card::CardUuid;
use crate::category::CategoryUuid;
use crate::topic::TopicUuid;
use crate::{Card, Category, Topic};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct FlashcardStore {
    cards: HashMap<CardUuid, Card>,
    topic: HashMap<TopicUuid, Topic>,
    categories: HashMap<CategoryUuid, Category>,
    topic_category_mapping: HashMap<TopicUuid, CategoryUuid>,
    card_topic_mapping: HashMap<CardUuid, TopicUuid>,
}

impl FlashcardStore {
    pub fn load_from_json<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);
        serde_json::from_reader(buffer).map_err(Into::into)
    }

    pub fn load_from_bytes<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);
        bincode::deserialize_from(buffer).map_err(Into::into)
    }

    pub fn save_to_json<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let file = OpenOptions::new().write(true).open(path)?;
        let buffer = BufWriter::new(file);
        serde_json::to_writer_pretty(buffer, self).map_err(Into::into)
    }

    pub fn save_to_bytes<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let file = OpenOptions::new().write(true).open(path)?;
        let buffer = BufWriter::new(file);
        bincode::serialize_into(buffer, self).map_err(Into::into)
    }
}

impl FlashcardStore {
    pub fn categories(&self) -> impl Iterator<Item = &Category> {
        self.categories.values()
    }

    pub fn topics(&self) -> impl Iterator<Item = &Topic> {
        self.topic.values()
    }

    pub fn topics_for<'a>(
        &'a self,
        uuid: &'a CategoryUuid,
    ) -> impl Iterator<Item = &'a Topic> + 'a {
        self.topic_category_mapping
            .iter()
            .filter_map(move |(topic, category)| (category == uuid).then_some(topic))
            .flat_map(|topic| self.topic.get(topic))
    }

    pub fn cards(&self) -> impl Iterator<Item = &Card> {
        self.cards.values()
    }

    pub fn cards_for<'a>(&'a self, uuid: &'a TopicUuid) -> impl Iterator<Item = &'a Card> + 'a {
        self.card_topic_mapping
            .iter()
            .flat_map(move |(card, topic)| (topic == uuid).then_some(card))
            .flat_map(|card| self.cards.get(card))
    }
}
