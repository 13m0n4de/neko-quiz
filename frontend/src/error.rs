use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuizError {
    #[error("Network error occurred: {0}")]
    NetworkError(#[from] gloo_net::Error),

    #[error("Failed to parse JSON: {0}")]
    ParseError(#[from] serde_json::Error),
}
