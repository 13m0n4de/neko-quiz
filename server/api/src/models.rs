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

#[derive(Deserialize, Serialize)]
pub struct General {
    pub title: String,
    pub return_score: bool,
}

#[derive(Deserialize)]
pub struct Config {
    pub general: General,
    pub questions: Vec<Question>,
    pub flag: Flag,
    pub message: Message,
    #[serde(default)]
    pub version: u64,
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
    #[serde(default)]
    pub version: u64,
}

#[derive(Deserialize, Clone)]
pub struct AnswerSubmission {
    pub answers: HashMap<Uuid, String>,
}

#[derive(Serialize)]
pub struct QuizResponse {
    pub status: bool,
    pub score: Option<u8>,
    pub message: String,
}
