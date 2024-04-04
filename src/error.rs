use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("syntax error: {0}")]
    Syntax(String),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error)
}
