use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use yew_bootstrap::util::Color;

#[derive(Deserialize, PartialEq)]
pub struct Quiz {
    pub title: String,
    pub questions: Vec<Question>,
    pub version: u64,
}

#[derive(Deserialize, PartialEq)]
pub struct Question {
    pub id: String,
    pub text: String,
    pub points: u32,
    pub hint: String,
}

#[derive(PartialEq, Clone)]
pub struct AlertInfo {
    pub color: Color,
    pub text: String,
}

#[derive(Serialize)]
pub struct AnswerSubmission {
    pub answers: HashMap<String, String>,
}

#[derive(Deserialize, PartialEq)]
pub struct QuizResponse {
    pub status: bool,
    pub score: Option<u8>,
    pub message: String,
}
