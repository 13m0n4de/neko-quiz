use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuizError {
    #[error("Failed to send the answers")]
    NetworkError(#[from] gloo_net::Error),

    #[error("Failed to parse the response")]
    ParseError(#[from] serde_json::Error),
}
