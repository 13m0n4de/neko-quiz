use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    models::{AnswerSubmission, Quiz, QuizResponse},
    state::AppState,
};

pub async fn get_quiz(State(state): State<Arc<AppState>>) -> Json<Quiz> {
    Json(Quiz {
        title: state.title().await,
        questions: state.questions().await,
        version: state.version().await,
    })
}

pub async fn create_submission(
    State(state): State<Arc<AppState>>,
    Json(submission): Json<AnswerSubmission>,
) -> Json<QuizResponse> {
    let mut status = true;
    let mut score = 0;

    for question in &state.questions().await {
        match submission.answers.get(&question.id) {
            Some(answer) if question.answers.contains(answer) => score += question.points,
            _ => status = false,
        }
    }

    let message = state.message().await;
    let flag = state.flag().await;

    let response_message = if status {
        message.correct.replace("$FLAG", &flag)
    } else {
        message.incorrect
    };

    let response = QuizResponse {
        status,
        score,
        message: response_message,
    };

    Json(response)
}
