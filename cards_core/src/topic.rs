use crate::{
    Add, AddItem, Card, Category, Create, Find, FindParent, HasParent, Remove, RemoveChild,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type TopicUuid = Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    uuid: TopicUuid,
    name: String,
    cards: Vec<Card>,
}

impl Topic {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn is(&self, uuid: &Uuid) -> bool {
        self.uuid == *uuid
    }

    pub fn cards(&self) -> impl Iterator<Item = &Card> {
        self.cards.iter()
    }
}

pub struct TopicOptions {
    pub name: String,
}

impl Create for Topic {
    type Options = TopicOptions;

    fn create(options: Self::Options) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: options.name,
            cards: vec![],
        }
    }
}

impl HasParent for Topic {
    type Parent = Category;
}
