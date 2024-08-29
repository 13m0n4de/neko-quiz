use std::{fs::File, io::Read, sync::Arc};
use tokio::sync::RwLock;

use crate::models::{Config, Message, Question};

pub struct AppState {
    config: Arc<RwLock<Config>>,
}

impl AppState {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        Self { config }
    }

    pub async fn title(&self) -> String {
        self.config.read().await.title.clone()
    }

    pub async fn questions(&self) -> Vec<Question> {
        let config = self.config.read().await;
        config.questions.clone()
    }

    pub async fn flag(&self) -> String {
        let config = self.config.read().await;
        if let Ok(flag) = std::env::var(&config.flag.env) {
            flag
        } else if let Ok(mut file) = File::open(&config.flag.file) {
            let mut flag = String::new();
            file.read_to_string(&mut flag)
                .expect("Unable to read flag file");
            flag
        } else {
            config.flag.static_str.clone()
        }
    }

    pub async fn message(&self) -> Message {
        self.config.read().await.message.clone()
    }
}
