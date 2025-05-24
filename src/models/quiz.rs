use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Quiz {
    pub title: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Question {
    #[serde(default)]
    pub id: String,
    pub text: String,
    pub points: u8,
    pub hint: String,
    #[serde(skip_serializing, default)]
    pub answers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuizResponse {
    pub status: bool,
    pub score: Option<u8>,
    pub message: String,
}
