use crate::{
    error::QuizError,
    models::{AnswerSubmission, Quiz, QuizResponse},
};
use gloo_net::http::Request;
use std::collections::HashMap;

pub async fn get_quiz() -> Result<Quiz, QuizError> {
    let response = Request::get("/api/quiz").send().await?;
    Ok(response.json::<Quiz>().await?)
}

pub async fn create_submission(
    answers: HashMap<String, String>,
) -> Result<QuizResponse, QuizError> {
    let answer_submission = AnswerSubmission { answers };

    let response = Request::post("/api/quiz/submission")
        .json(&answer_submission)?
        .send()
        .await?;

    Ok(response.json().await?)
}
