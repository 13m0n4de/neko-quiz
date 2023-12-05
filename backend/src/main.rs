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

    #[clap(short = 'c', long = "config", default_value = "config.json")]
    config: String,

    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,
}

#[derive(Deserialize, Clone)]
struct Question {
    text: String,
    points: u8,
    hint: String,
    answer: Vec<String>,
}

#[derive(Deserialize, Clone)]
struct Config {
    title: String,
    questions: Vec<Question>,
}

#[derive(Serialize, Clone)]
struct Info {
    title: String,
    questions: Vec<QuestionWithoutAnswer>,
}

#[derive(Serialize, Clone)]
struct QuestionWithoutAnswer {
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

static INFO: OnceLock<Info> = OnceLock::new();
static ANSWERS: OnceLock<HashMap<usize, (Vec<String>, u8)>> = OnceLock::new();

async fn get_info() -> Json<Info> {
    let info = INFO.get().unwrap();
    Json(info.clone())
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

fn init_config(config_path: &str) {
    let file = File::open(config_path).expect("Unable to open questions file");
    let config: Config = serde_json::from_reader(file).expect("Unable to parse questions file");

    INFO.get_or_init(|| {
        let questions = config
            .questions
            .iter()
            .cloned()
            .enumerate()
            .map(|(id, question)| QuestionWithoutAnswer {
                id: id + 1,
                text: markdown::to_html(&question.text),
                points: question.points,
                hint: markdown::to_html(&question.hint),
            })
            .collect();
        Info {
            title: config.title,
            questions,
        }
    });

    ANSWERS.get_or_init(|| {
        let mut answers_map = HashMap::new();
        config
            .questions
            .iter()
            .cloned()
            .enumerate()
            .for_each(|(id, question)| {
                answers_map.insert(id + 1, (question.answer, question.points));
            });
        answers_map
    });
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level));
    }
    tracing_subscriber::fmt::init();

    init_config(&opt.config);

    let app = Router::new()
        .nest_service("/", ServeDir::new(&opt.static_dir))
        .route("/api/info", get(get_info))
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
