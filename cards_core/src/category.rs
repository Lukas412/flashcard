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

impl AddParent for Category {}

impl Add<Topic> for Category {}

impl AddItem<Topic> for Category {
    fn add_item(&mut self, item: Topic) {
        self.topics.push(item)
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

impl HasParent for Category {
    type Parent = Self;
}
