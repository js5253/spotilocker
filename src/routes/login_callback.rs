use anyhow::anyhow;
use axum::{
    Error,
    body::Body,
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use base64::{engine::general_purpose::STANDARD, prelude::*};
use log::info;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::AppState;
#[derive(Deserialize)]
pub struct OAuthCallbackResponse {
    code: String,
    state: String,
    error: Option<String>,
}
#[derive(Deserialize)]
pub struct SpotifyRequestBody {
    code: String,
    redirect_uri: String,
    grant_type: String,
}
#[axum::debug_handler]
pub async fn login_callback(
    params: Query<OAuthCallbackResponse>,
    State(app_state): State<AppState>,
) -> axum::response::Result<String, Response> {
    if params.state.is_empty() {
        return Err((StatusCode::UNPROCESSABLE_ENTITY).into_response());
    }
    if let Some(error) = &params.error {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, ()).into_response());
    }
    let param = [
        ("code", params.code.clone()),
        ("redirect_uri", format!("{}callback", app_state.app_url)),
        ("grant_type", "authorization_code".to_string()),
    ];
    info!("Sending req soon...!");
    let auth_header = format!(
        "Basic {}",
        STANDARD.encode(format!(
            "{}:{}",
            app_state.client_id, app_state.client_secret
        ))
    );
    let req = app_state
        .http_client
        .post("https://accounts.spotify.com/api/token")
        .form(&param)
        .header("Content-type", "application/x-www-form-urlencoded")
        .header("Authorization", auth_header)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e}")).into_response())?;

    let body = req
        .error_for_status()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e}")).into_response())?
        .text()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e}")).into_response())?;
    Ok(body.to_string())
    // let req = match req {
    //     Ok(val) => {
    //         if val.status() == 400 Err("400 Forbdden Response")
    //     },
    //     Err(e) => format!("Hi"),
    // }
}
