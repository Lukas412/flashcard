use crate::{Add, AddChild, AddItem, AddParent, Card, Create, Find, HasParent, Remove, Topic};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type CategoryUuid = Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    uuid: CategoryUuid,
    name: String,
    categories: Vec<Self>,
    topics: Vec<Topic>,
}

pub struct CategoryOptions {
    pub name: String,
}

impl Create for Category {
    type Options = CategoryOptions;

    fn create(options: Self::Options) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: options.name,
            categories: vec![],
            topics: vec![],
        }
    }
}

impl Add<Self> for Category {}

impl AddItem<Self> for Category {
    fn add_item(&mut self, item: Self) {
        self.categories.push(item)
    }
}

impl AddChild<Self> for Category {}

impl AddParent for Category {}

impl Add<Topic> for Category {}

impl AddItem<Topic> for Category {
    fn add_item(&mut self, item: Topic) {
        self.topics.push(item)
    }
}

impl AddChild<Topic> for Category {}

impl AddChild<Card> for Category {}

impl Remove<Self> for Category {
    fn remove(&mut self, uuid: &Uuid) -> Option<Self> {
        self.categories()
            .enumerate()
            .find_map(|(index, card)| card.is(uuid).then_some(index))
            .map(|index| self.categories.swap_remove(index))
            .or_else(|| {
                self.categories
                    .iter_mut()
                    .find_map(|category| category.remove(uuid))
            })
    }
}

impl Remove<Topic> for Category {
    fn remove(&mut self, uuid: &Uuid) -> Option<Topic> {
        self.topics()
            .enumerate()
            .find_map(|(index, topic)| topic.is(uuid).then_some(index))
            .map(|index| self.topics.swap_remove(index))
            .or_else()
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

impl Find<Self> for Category {
    fn find(&self, uuid: &Uuid) -> Option<&Self> {
        self.is(uuid)
            .then_some(self)
            .or_else(|| self.categories().find_map(|category| category.find(uuid)))
    }

    fn find_mut(&mut self, uuid: &Uuid) -> Option<&mut Self> {
        self.is(uuid).then_some(self).or_else(|| {
            self.categories
                .iter_mut()
                .find_map(|category| category.find_mut(uuid))
        })
    }
}

impl Find<Topic> for Category {
    fn find(&self, uuid: &Uuid) -> Option<&Topic> {
        self.topics()
            .find(|topic| topic.is(uuid))
            .or_else(|| self.categories().find_map(|category| category.find(uuid)))
    }

    fn find_mut(&mut self, uuid: &Uuid) -> Option<&mut Topic> {
        self.topics
            .iter_mut()
            .find(|topic| topic.is(uuid))
            .or_else(|| {
                self.categories
                    .iter_mut()
                    .find_map(|category| category.find_mut(uuid))
            })
    }
}

impl Find<Card> for Category {
    fn find(&self, uuid: &Uuid) -> Option<&Card> {
        self.topics()
            .find_map(|topic| topic.find(uuid))
            .or_else(|| self.categories().find_map(|category| category.find(uuid)))
    }

    fn find_mut(&mut self, uuid: &Uuid) -> Option<&mut Card> {
        self.topics
            .iter_mut()
            .find_map(|topic| topic.find_mut(uuid))
            .or_else(|| {
                self.categories
                    .iter_mut()
                    .find_map(|category| category.find_mut(uuid))
            })
    }
}

impl HasParent for Category {
    type Parent = Self;
}
