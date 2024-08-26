use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    models::{AnswerSubmission, Quiz, QuizResponse},
    state::AppState,
};

pub async fn get_quiz(State(state): State<Arc<AppState>>) -> Json<Quiz> {
    Json(Quiz {
        title: state.title.clone(),
        questions: state.questions.clone(),
    })
}

pub async fn submit_answers(
    State(state): State<Arc<AppState>>,
    Json(submission): Json<AnswerSubmission>,
) -> Json<QuizResponse> {
    let mut status = true;
    let mut score = 0;

    for question in &state.questions {
        match submission.answers.get(&question.id) {
            Some(answer) if question.answers.contains(answer) => score += question.points,
            _ => status = false,
        }
    }

    let message = if status {
        state.message.correct.replace("$FLAG", &state.flag)
    } else {
        state.message.incorrect.clone()
    };

    let response = QuizResponse {
        status,
        score,
        message,
    };

    Json(response)
}
