use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    uuid: Uuid,
    question: String,
    answer: String,
}

impl Card {
    pub fn create(question: String, answer: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            question,
            answer,
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn question(&self) -> &str {
        self.question.as_str()
    }

    pub fn answer(&self) -> &str {
        self.answer.as_str()
    }
}
