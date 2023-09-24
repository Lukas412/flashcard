use crate::{Card, Topic};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    uuid: Uuid,
    name: String,
    categories: Vec<Self>,
    topics: Vec<Topic>,
}

impl Category {
    pub fn create(name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            categories: vec![],
            topics: vec![],
        }
    }

    pub fn wrap(self, name: String) -> Self {
        let mut category = Self::create(name);
        category.add_category(self);
        category
    }

    pub fn create_category(&mut self, name: String) {
        self.add_category(Self::create(name))
    }

    pub fn create_topic(&mut self, name: String) {
        self.add_topic(Topic::create(name))
    }
}

impl Category {
    pub fn create_topic_under(&mut self, parent: Uuid, name: String) {
        for category in self.categories.iter_mut() {
            if category.uuid == parent {
                
            }
        }
    }
}

impl Category {
    pub fn remove_card(&mut self, uuid: &Uuid) -> Option<Card> {
        self.topics
            .iter_mut()
            .find_map(|topic| topic.remove_card(uuid))
            .or_else(|| {
                self.categories
                    .iter_mut()
                    .find_map(|category| category.remove_card(uuid))
            })
    }

    pub fn remove_topic(&mut self, uuid: &Uuid) -> Option<Topic> {
        self.topics
            .iter()
            .enumerate()
            .find_map(|(index, topic)| (topic.uuid() == uuid).then_some(index))
            .map(|index| self.topics.swap_remove(index))
            .or_else(|| {
                self.categories
                    .iter_mut()
                    .find_map(|category| category.remove_topic(uuid))
            })
    }

    pub fn remove_category(&mut self, uuid: &Uuid) -> Option<Self> {
        self.categories
            .iter()
            .enumerate()
            .find_map(|(index, category)| (category.is(uuid)).then_some(index))
            .map(|index| self.categories.swap_remove(index))
            .or_else(|| {
                self.categories
                    .iter_mut()
                    .find_map(|category| category.remove_category(uuid))
            })
    }
}

impl Category {
    pub fn is(&self, uuid: &Uuid) -> bool {
        self.uuid == *uuid
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn categories(&self) -> impl Iterator<Item = &Self> {
        self.categories.iter()
    }

    pub fn topics(&self) -> impl Iterator<Item = &Topic> {
        self.topics.iter()
    }
}

impl Category {
    fn add_category(&mut self, category: Self) {
        self.categories.push(category)
    }

    pub(crate) fn add_topic(&mut self, topic: Topic) {
        self.topics.push(topic)
    }
}
