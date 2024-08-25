use crate::{
    error::QuizError,
    models::{Answer, Info, QuizResponse},
};
use gloo_net::http::Request;
use std::collections::HashMap;

pub async fn get_info() -> Result<Info, QuizError> {
    let response = Request::get("/api/info").send().await?;
    Ok(response.json::<Info>().await?)
}

pub async fn submit_answers(answers: HashMap<String, String>) -> Result<QuizResponse, QuizError> {
    let answers_data: Vec<Answer> = answers
        .into_iter()
        .map(|(id, answer)| Answer { id, answer })
        .collect();

    let response = Request::post("/api/answers")
        .json(&answers_data)?
        .send()
        .await?;

    Ok(response.json().await?)
}
