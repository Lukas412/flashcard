use crate::card::CardUuid;
use crate::category::CategoryUuid;
use crate::topic::TopicUuid;
use crate::{Card, Category, Topic};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct FlashCardStore {
    cards: HashMap<CardUuid, Card>,
    topic: HashMap<TopicUuid, Topic>,
    categories: HashMap<CategoryUuid, Category>,
    topic_category_mapping: HashMap<TopicUuid, CategoryUuid>,
    card_topic_mapping: HashMap<CardUuid, TopicUuid>,
}

impl FlashCardStore {
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

impl FlashCardStore {
    pub fn categories(&self) -> impl Iterator<Item = (&CategoryUuid, &Category)> {
        self.categories.iter()
    }

    pub fn topics(&self) -> impl Iterator<Item = &Topic> {
        self.topic.values()
    }

    pub fn topics_for<'a>(
        &'a self,
        uuid: &'a CategoryUuid,
    ) -> impl Iterator<Item = (&TopicUuid, &'a Topic)> + 'a {
        self.topic_category_mapping
            .iter()
            .filter_map(move |(topic, category)| (category == uuid).then_some(topic))
            .flat_map(|uuid| self.topic.get(uuid).map(|topic| (uuid, topic)))
    }

    pub fn cards(&self) -> impl Iterator<Item = &Card> {
        self.cards.values()
    }

    pub fn cards_for<'a>(
        &'a self,
        uuid: &'a TopicUuid,
    ) -> impl Iterator<Item = (&'a Uuid, &'a Card)> + 'a {
        self.card_topic_mapping
            .iter()
            .flat_map(move |(card, topic)| (topic == uuid).then_some(card))
            .flat_map(|uuid| self.cards.get(uuid).map(|card| (uuid, card)))
    }
}

impl FlashCardStore {
    pub fn remove_category(
        &mut self,
        uuid: &Uuid,
    ) -> Option<(
        CategoryUuid,
        Category,
        Vec<(TopicUuid, Topic, Vec<(CardUuid, Card)>)>,
    )> {
        let topics = self
            .topic_category_mapping
            .iter()
            .filter_map(|(topic_uuid, category_uuid)| (category_uuid == uuid).then_some(topic_uuid))
            .copied()
            .collect::<Vec<_>>()
            .iter()
            .filter_map(|uuid| self.remove_topic(&uuid))
            .collect();
        self.categories
            .remove(uuid)
            .map(|category| (uuid.clone(), category, topics))
    }

    pub fn remove_topic(
        &mut self,
        uuid: &TopicUuid,
    ) -> Option<(TopicUuid, Topic, Vec<(CardUuid, Card)>)> {
        let cards = self
            .card_topic_mapping
            .iter()
            .filter_map(|(card_uuid, topic_uuid)| (topic_uuid == uuid).then_some(card_uuid))
            .copied()
            .collect::<Vec<_>>()
            .iter()
            .filter_map(|uuid| self.remove_card(uuid))
            .collect();
        self.topic_category_mapping.remove(uuid);
        self.topic
            .remove(uuid)
            .map(|topic| (uuid.clone(), topic, cards))
    }

    pub fn remove_card(&mut self, uuid: &CardUuid) -> Option<(CardUuid, Card)> {
        self.card_topic_mapping.remove(uuid);
        self.cards.remove(uuid).map(|card| (uuid.clone(), card))
    }
}
