use axum::{
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;
use std::{fs::File, net::Ipv4Addr, str::FromStr};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

#[derive(Parser, Debug)]
#[clap(name = "neko-quiz-backend", about = "backend for neko quiz")]
struct Opt {
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    #[clap(short = 'a', long = "addr", default_value = "localhost")]
    addr: String,

    #[clap(short = 'p', long = "port", default_value = "3000")]
    port: u16,

    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,

    #[clap(long = "questions", default_value = "questions.json")]
    questions: String,
}

#[derive(Deserialize, Clone)]
struct Question {
    text: String,
    points: u8,
    hint: String,
    answer: Vec<String>,
}

#[derive(Serialize, Clone)]
struct QuestionResponse {
    id: usize,
    text: String,
    points: u8,
    hint: String,
}

#[derive(Deserialize)]
struct AnswerRequest {
    id: usize,
    answer: String,
}

#[derive(Serialize)]
struct AnswerResponse {
    status: bool,
    score: u8,
    message: String,
}

static QUESTIONS: OnceLock<Vec<QuestionResponse>> = OnceLock::new();
static ANSWERS: OnceLock<HashMap<usize, (Vec<String>, u8)>> = OnceLock::new();

async fn get_questions() -> Json<Vec<QuestionResponse>> {
    let questions = QUESTIONS.get().unwrap();
    Json(questions.clone())
}

async fn submit_answers(Json(user_answers): Json<Vec<AnswerRequest>>) -> Json<AnswerResponse> {
    let correct_answers = ANSWERS.get().unwrap();

    let mut status = true;
    let mut score = 0;

    for user_answer in user_answers {
        if let Some((correct_answer, points)) = correct_answers.get(&user_answer.id) {
            if correct_answer.contains(&user_answer.answer) {
                score += points;
            } else {
                status = false;
            }
        }
    }

    let message = if status {
        std::env::var("GZCTF_FLAG").unwrap_or("flag{test_flag}".to_string())
    } else {
        "没有全部答对，不能给你 FLAG 哦。".to_string()
    };

    let response = AnswerResponse {
        status,
        score,
        message,
    };

    Json(response)
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level));
    }
    tracing_subscriber::fmt::init();

    let file = File::open(&opt.questions).expect("Unable to open questions file");
    let questions: Vec<Question> =
        serde_json::from_reader(file).expect("Unable to parse questions file");

    QUESTIONS.get_or_init(|| {
        questions
            .iter()
            .cloned()
            .enumerate()
            .map(|(id, question)| QuestionResponse {
                id: id + 1,
                text: question.text,
                points: question.points,
                hint: question.hint,
            })
            .collect()
    });

    ANSWERS.get_or_init(|| {
        let mut answers_map = HashMap::new();
        questions
            .iter()
            .cloned()
            .enumerate()
            .for_each(|(id, question)| {
                answers_map.insert(id + 1, (question.answer, question.points));
            });
        answers_map
    });

    let app = Router::new()
        .nest_service("/", ServeDir::new(&opt.static_dir))
        .route("/api/questions", get(get_questions))
        .route("/api/answers", post(submit_answers))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = (
        Ipv4Addr::from_str(&opt.addr).unwrap_or(Ipv4Addr::LOCALHOST),
        opt.port,
    );

    tracing::info!("listening on http://{}:{}", addr.0, addr.1);

    let listener = TcpListener::bind(addr)
        .await
        .expect("Unable to bind socket address");

    axum::serve(listener, app)
        .await
        .expect("Unable to start server");
}
