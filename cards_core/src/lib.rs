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
    pub fn new(uuid: Uuid, name: String, categories: Vec<Self>, topics: Vec<Topic>) -> Self {
        Self {
            uuid,
            name,
            categories,
            topics,
        }
    }

    pub fn create(name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            categories: vec![],
            topics: vec![],
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn add_category(&mut self, category: Self) {
        self.categories.push(category)
    }

    pub fn add_topic(&mut self, topic: Topic) {
        self.topics.push(topic)
    }

    pub fn categories(&self) -> impl Iterator<Item = &Self> {
        self.categories.iter()
    }

    pub fn topics(&self) -> impl Iterator<Item = &Topic> {
        self.topics.iter()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    uuid: Uuid,
    name: String,
    cards: Vec<Card>,
}

impl Topic {
    pub fn new(uuid: Uuid, name: String, cards: Vec<Card>) -> Self {
        Self { uuid, name, cards }
    }

    pub fn create(name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            cards: vec![],
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    uuid: Uuid,
    question: String,
    answer: String,
}

impl Card {
    pub fn new(uuid: Uuid, question: String, answer: String) -> Self {
        Self {
            uuid,
            question,
            answer,
        }
    }

    pub fn create(question: String, answer: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            question,
            answer,
        }
    }

    pub fn question(&self) -> &str {
        self.question.as_str()
    }

    pub fn answer(&self) -> &str {
        self.answer.as_str()
    }
}
