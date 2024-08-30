#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod handlers;
mod models;
mod state;

use axum::{
    routing::{get, post},
    Router,
};
use std::{path::Path, sync::Arc};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};

use handlers::{create_submission, get_quiz};
use state::AppState;

pub use models::Config;

pub fn build_router<P: AsRef<Path>>(config: Arc<RwLock<Config>>, serve_dir: P) -> Router {
    let state = Arc::new(AppState::new(config));

    Router::new()
        .nest_service("/", ServeDir::new(serve_dir))
        .route("/api/quiz", get(get_quiz))
        .route("/api/quiz/submission", post(create_submission))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(state)
}
