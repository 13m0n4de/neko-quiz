use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct Config {
    pub title: String,
    pub questions: Vec<Question>,
    pub flag: Flag,
    pub message: Message,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Question {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub text: String,
    pub points: u8,
    pub hint: String,
    #[serde(skip_serializing)]
    pub answers: Vec<String>,
}

#[derive(Serialize)]
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
