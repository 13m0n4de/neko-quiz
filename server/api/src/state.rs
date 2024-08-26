use std::{fs::File, io::Read};

use crate::models::{Config, Message, Question};

pub struct AppState {
    pub title: String,
    pub questions: Vec<Question>,
    pub flag: String,
    pub message: Message,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let mut questions = Vec::new();

        for q_config in config.questions {
            let question = Question::from(q_config.clone());
            questions.push(question);
        }

        let flag = if let Ok(flag) = std::env::var(config.flag.env) {
            flag
        } else if let Ok(mut file) = File::open(config.flag.file) {
            let mut flag = String::new();
            file.read_to_string(&mut flag)
                .expect("Unable to read flag file");
            flag
        } else {
            config.flag.static_str
        };
        let message = config.message;

        let title = config.title;

        Self {
            title,
            questions,
            flag,
            message,
        }
    }
}
