use std::fmt::Display;

use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use thiserror::Error;
use tracing::instrument;

#[derive(Error, Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error)
}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::DatabaseError(error) => f.write_str(&error.to_string()),
        }
    }
}

impl IntoResponse for AppError {
    #[instrument]
    fn into_response(self) -> Response {
        match self {
            AppError::DatabaseError(err) => {
                // Log the exact error internally for debugging
                tracing::error!("Database error: {:?}", err);
                
                // Return a safe, generic message to the client
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong with the database",
                )
                    .into_response()
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}
