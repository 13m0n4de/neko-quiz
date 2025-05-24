use crate::models::quiz::Question;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct General {
    pub title: String,
    pub return_score: bool,
}

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
    pub general: General,
    pub questions: Vec<Question>,
    pub flag: Flag,
    pub message: Message,
}
