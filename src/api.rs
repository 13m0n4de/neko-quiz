use crate::models::quiz::{Quiz, QuizResponse};
use leptos::prelude::*;
use server_fn::codec::GetUrl;
use std::collections::HashMap;

#[cfg(feature = "ssr")]
use crate::models::config::Config;
#[cfg(feature = "ssr")]
use std::sync::Arc;

#[server(GetQuiz, prefix = "/api", endpoint = "quiz", input = GetUrl)]
pub async fn get_quiz() -> Result<Quiz, ServerFnError> {
    let config = expect_context::<Arc<Config>>();

    Ok(Quiz {
        title: config.general.title.clone(),
        questions: config.questions.clone(),
    })
}

#[server(CreateSubmission, prefix = "/api/quiz", endpoint = "submission")]
pub async fn create_submission(
    answers: HashMap<String, String>,
) -> Result<QuizResponse, ServerFnError> {
    let config = expect_context::<Arc<Config>>();

    let mut status = true;
    let mut score = 0;

    for question in &config.questions {
        match answers.get(&question.id) {
            Some(answer) if question.answers.contains(answer) => score += question.points,
            _ => status = false,
        }
    }

    let flag = if let Ok(flag) = std::env::var(&config.flag.env) {
        flag
    } else if let Ok(flag) = tokio::fs::read_to_string(&config.flag.file).await {
        flag.trim().to_string()
    } else {
        config.flag.static_str.clone()
    };

    let response_message = if status {
        config.message.correct.replace("$FLAG", &flag)
    } else {
        config.message.incorrect.clone()
    };

    Ok(QuizResponse {
        status,
        score: if config.general.return_score {
            Some(score)
        } else {
            None
        },
        message: response_message,
    })
}
