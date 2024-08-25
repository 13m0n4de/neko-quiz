use crate::models::{Answer, Info, QuizResponse};
use gloo_net::http::Request;
use std::collections::HashMap;

pub async fn get_info() -> Result<Info, String> {
    let response = Request::get("/api/info")
        .send()
        .await
        .map_err(|_| "Failed to get page information.")?;

    response
        .json::<Info>()
        .await
        .map_err(|_| "Failed to parse server response.".to_string())
}

pub async fn submit_answers(answers: HashMap<String, String>) -> Result<QuizResponse, String> {
    let answers_data: Vec<Answer> = answers
        .into_iter()
        .map(|(id, answer)| Answer { id, answer })
        .collect();

    let request = Request::post("/api/answers")
        .json(&answers_data)
        .map_err(|_| "Failed to serialize JSON.")?;

    let response = request.send().await.map_err(|_| "Failed to send answer.")?;

    response
        .json::<QuizResponse>()
        .await
        .map_err(|_| "Failed to parse server response.".to_string())
}
