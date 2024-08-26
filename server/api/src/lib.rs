#![warn(clippy::all, clippy::pedantic)]

mod handlers;
mod models;
mod state;

use axum::{
    routing::{get, post},
    Router,
};
use std::{path::PathBuf, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};

use handlers::{get_quiz, submit_answers};
use state::AppState;

pub use models::Config;

pub fn build_router(config: Config, serve_dir: PathBuf) -> Router {
    let state = Arc::new(AppState::new(config));

    Router::new()
        .nest_service("/", ServeDir::new(serve_dir))
        .route("/api/info", get(get_quiz))
        .route("/api/answers", post(submit_answers))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(state)
}
