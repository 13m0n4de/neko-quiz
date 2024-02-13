use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};

use std::io::Read;
use std::sync::OnceLock;

#[derive(Deserialize, Clone)]
struct Question {
    text: String,
    points: u8,
    hint: String,
    answer: Vec<String>,
}

#[derive(Deserialize, Clone)]
struct Flag {
    flag_env: String,
    flag_file: String,
    flag_static: String,
}

#[derive(Deserialize, Clone)]
struct Message {
    incorrect: String,
    correct: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    title: String,
    questions: Vec<Question>,
    flag: Flag,
    message: Message,
}

#[derive(Serialize, Clone)]
struct Info {
    title: String,
    questions: Vec<QuestionWithoutAnswer>,
}

#[derive(Serialize, Clone)]
struct QuestionWithoutAnswer {
    id: String,
    text: String,
    points: u8,
    hint: String,
}

#[derive(Deserialize, Clone)]
struct AnswerRequest {
    id: String,
    answer: String,
}

#[derive(Serialize)]
struct AnswerResponse {
    status: bool,
    score: u8,
    message: String,
}

static INFO: OnceLock<Info> = OnceLock::new();
static ANSWERS: OnceLock<HashMap<String, (Vec<String>, u8)>> = OnceLock::new();
static FLAG: OnceLock<String> = OnceLock::new();
static MESSAGE: OnceLock<Message> = OnceLock::new();

async fn get_info() -> Json<Info> {
    let info = INFO.get().unwrap();
    Json(info.clone())
}

async fn submit_answers(Json(request_answers): Json<Vec<AnswerRequest>>) -> Json<AnswerResponse> {
    let correct_answers = ANSWERS.get().unwrap();
    let user_answers: HashMap<String, String> = HashMap::from_iter(
        request_answers
            .iter()
            .cloned()
            .map(|AnswerRequest { id, answer }| (id, answer)),
    );

    let mut status = true;
    let mut score = 0;

    for (id, (correct_answer, points)) in correct_answers {
        match user_answers.get(id) {
            Some(answer) if correct_answer.contains(answer) => score += points,
            _ => status = false,
        }
    }

    let message = if status {
        MESSAGE
            .get()
            .unwrap()
            .correct
            .replace("$FLAG", FLAG.get().unwrap())
    } else {
        MESSAGE.get().unwrap().incorrect.clone()
    };

    let response = AnswerResponse {
        status,
        score,
        message,
    };

    Json(response)
}

fn init(config: Config) {
    let mut questions = vec![];
    let mut answers_map = HashMap::new();

    for (idx, question) in config.questions.iter().enumerate() {
        let question_id = format!("q{}", idx + 1);
        questions.push(QuestionWithoutAnswer {
            id: question_id.clone(),
            text: question.text.clone(),
            points: question.points,
            hint: question.hint.clone(),
        });
        answers_map.insert(question_id, (question.answer.clone(), question.points));
    }

    INFO.get_or_init(|| Info {
        title: config.title,
        questions,
    });

    ANSWERS.get_or_init(|| answers_map);

    FLAG.get_or_init(|| {
        if let Ok(flag) = std::env::var(config.flag.flag_env) {
            flag
        } else if let Ok(mut file) = File::open(config.flag.flag_file) {
            let mut flag = String::new();
            file.read_to_string(&mut flag)
                .expect("Unable to read flag file");
            flag
        } else {
            config.flag.flag_static
        }
    });

    MESSAGE.get_or_init(|| config.message);
}

pub fn build_router(config: Config, serve_dir: &str) -> Router {
    init(config);

    Router::new()
        .nest_service("/", ServeDir::new(serve_dir))
        .route("/api/info", get(get_info))
        .route("/api/answers", post(submit_answers))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}
