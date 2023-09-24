use crate::{Create, HasParent, Topic};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    uuid: Uuid,
    question: String,
    answer: String,
}

impl Card {
    pub fn is(&self, uuid: &Uuid) -> bool {
        self.uuid == *uuid
    }

    pub fn question(&self) -> &str {
        self.question.as_str()
    }

    pub fn answer(&self) -> &str {
        self.answer.as_str()
    }
}

pub struct CardOptions {
    pub question: String,
    pub answer: String,
}

impl Create for Card {
    type Options = CardOptions;

    fn create(options: Self::Options) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            question: options.question,
            answer: options.answer,
        }
    }
}

impl HasParent for Card {
    type Parent = Topic;
}
