use crate::{Card, Category};
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

impl Topic {
    pub fn create(name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            cards: vec![],
        }
    }

    pub fn wrap(self, name: String) -> Category {
        let mut category = Category::create(name);
        category.add_topic(self);
        category
    }

    pub fn create_card(&mut self, question: String, answer: String) {
        self.add_card(Card::create(question, answer))
    }
}

impl Topic {
    pub fn remove_card(&mut self, uuid: &Uuid) -> Option<Card> {
        self.cards
            .iter()
            .enumerate()
            .find_map(|(index, card)| (card.uuid() == uuid).then_some(index))
            .map(|index| self.cards.swap_remove(index))
    }
}

impl Topic {
    fn add_card(&mut self, card: Card) {
        self.cards.push(card)
    }
}
