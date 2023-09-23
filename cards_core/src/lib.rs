use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    uuid: Uuid,
    name: String,
    topics: Vec<Topic>,
}

impl Category {
    pub fn new(uuid: Uuid, name: String, topics: Vec<Topic>) -> Self {
        Self { uuid, name, topics }
    }

    pub fn create(name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            topics: vec![],
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn add_topic(&mut self, topic: Topic) {
        self.topics.push(topic)
    }
}

impl IntoIterator for Category {
    type Item = Topic;
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.topics.into_iter()
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
