use axum::{extract::State, response::{IntoResponse, Response}};
use reqwest::StatusCode;

use crate::{AppState, api::{fetch_token, get_liked_songs}};

pub async fn refresh_library(State(app_state): State<AppState>) -> axum::response::Result<String, Response> {
    let token = fetch_token(&app_state).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e}")).into_response())?;
    let liked_songs = get_liked_songs(&app_state.http_client, 1, token);
    println!("{:?}", liked_songs.await);

    Ok(String::from("ASSA"))
}
