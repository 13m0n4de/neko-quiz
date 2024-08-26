use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Clone)]
pub struct Flag {
    pub env: String,
    pub file: String,
    pub static_str: String,
}

#[derive(Deserialize, Clone)]
pub struct Message {
    pub incorrect: String,
    pub correct: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub title: String,
    pub questions: Vec<QuestionConfig>,
    pub flag: Flag,
    pub message: Message,
}

#[derive(Deserialize, Clone)]
pub struct QuestionConfig {
    pub text: String,
    pub points: u8,
    pub hint: String,
    pub answers: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Question {
    pub id: Uuid,
    pub text: String,
    pub points: u8,
    pub hint: String,
    #[serde(skip_serializing)]
    pub answers: Vec<String>,
}

impl From<QuestionConfig> for Question {
    fn from(config: QuestionConfig) -> Self {
        Self {
            id: Uuid::new_v4(),
            text: config.text,
            points: config.points,
            hint: config.hint,
            answers: config.answers,
        }
    }
}
#[derive(Serialize, Clone)]
pub struct Quiz {
    pub title: String,
    pub questions: Vec<Question>,
}

#[derive(Deserialize, Clone)]
pub struct AnswerSubmission {
    pub answers: HashMap<Uuid, String>,
}

#[derive(Serialize)]
pub struct QuizResponse {
    pub status: bool,
    pub score: u8,
    pub message: String,
}
