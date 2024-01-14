use axum::{
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::sync::OnceLock;
use std::{collections::HashMap, usize};
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
struct Config {
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
    id: usize,
    text: String,
    points: u8,
    hint: String,
}

#[derive(Deserialize, Clone)]
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
static FLAG: OnceLock<String> = OnceLock::new();
static MESSAGE: OnceLock<Message> = OnceLock::new();

async fn get_info() -> Json<Info> {
    let info = INFO.get().unwrap();
    Json(info.clone())
}

async fn submit_answers(Json(request_answers): Json<Vec<AnswerRequest>>) -> Json<AnswerResponse> {
    let correct_answers = ANSWERS.get().unwrap();
    let user_answers: HashMap<usize, String> = HashMap::from_iter(
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

fn init(config_path: &str) {
    let file = File::open(config_path).expect("Unable to open config file");
    let config: Config = serde_json::from_reader(file).expect("Unable to parse config file");

    let mut questions = vec![];
    let mut answers_map = HashMap::new();

    for (id, question) in config.questions.iter().enumerate() {
        questions.push(QuestionWithoutAnswer {
            id: id + 1,
            text: question.text.clone(),
            points: question.points.clone(),
            hint: question.hint.clone(),
        });
        answers_map.insert(id + 1, (question.answer.clone(), question.points));
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

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level));
    }
    tracing_subscriber::fmt::init();

    init(&opt.config);

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
