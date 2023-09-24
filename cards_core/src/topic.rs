use crate::{
    Add, AddItem, Card, Category, Create, Find, FindParent, HasParent, Remove, RemoveChild,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    uuid: Uuid,
    name: String,
    cards: Vec<Card>,
}

impl Topic {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
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

impl Add<Card> for Topic {}

impl AddItem<Card> for Topic {
    fn add_item(&mut self, item: Card) {
        self.cards.push(item)
    }
}

impl Find<Card> for Topic {
    fn find(&self, uuid: &Uuid) -> Option<&Card> {
        self.cards().find(|card| card.is(uuid))
    }

    fn find_mut(&mut self, uuid: &Uuid) -> Option<&mut Card> {
        self.cards.iter_mut().find(|card| card.is(uuid))
    }
}

impl FindParent<Card> for Topic {
    fn find_parent(&self, uuid: &Uuid) -> Option<&<Card as HasParent>::Parent> {
        self.find(uuid).map(|_| self)
    }

    fn find_parent_mut(&mut self, uuid: &Uuid) -> Option<&mut <Card as HasParent>::Parent> {
        self.find(uuid).map(|_| self)
    }
}

impl Remove<Card> for Topic {
    fn remove(&mut self, uuid: &Uuid) -> Option<Card> {
        self.cards()
            .enumerate()
            .find_map(|(index, card)| card.is(uuid).then_some(index))
            .map(|index| self.cards.swap_remove(index))
    }
}

impl RemoveChild<Card> for Topic {}

impl HasParent for Topic {
    type Parent = Category;
}
