use serde::{Deserialize, Serialize};
use yew_bootstrap::util::Color;

#[derive(Deserialize, PartialEq, Clone)]
pub struct Info {
    pub title: String,
    pub questions: Vec<Question>,
}

#[derive(Deserialize, PartialEq, Clone)]
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
pub struct Answer {
    pub id: String,
    pub answer: String,
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct QuizResponse {
    pub status: bool,
    pub score: u8,
    pub message: String,
}
