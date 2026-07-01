use axum::{Json, extract::State, http::Result, response::Response};

use crate::{AppState, errors::AppError, models::Track};
#[axum::debug_handler]
pub async fn top_songs(app_state: State<AppState>) -> axum::response::Result<Json<Vec<Track>>, AppError> {
    let songs: Vec<Track> = sqlx::query_as("SELECT * FROM tracks ORDER BY play_count DESC LIMIT 5").fetch_all(&app_state.db).await?;
    
    Ok(Json(songs))
}