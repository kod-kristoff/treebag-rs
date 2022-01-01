use std::result;


#[derive(Debug, thiserror::Error, PartialEq)]
pub enum AppError {
    #[error("Unknown command: {0}")]
    UnknownCommand(String),
}

#[derive(Debug, PartialEq)]
pub enum Response<T> {
    Exit,
    Ok(T),
    Err(AppError),
}

pub type Result<T> = result::Result<T, AppError>;
