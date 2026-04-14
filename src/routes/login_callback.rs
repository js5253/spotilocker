use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use base64::{engine::general_purpose::STANDARD, prelude::*};
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
) -> Response {
    if params.state.is_empty() {
        return Response::builder()
            .status(500)
            .body(Body::from(()))
            .unwrap();
    }
    if let Some(error) = &params.error {
        return Response::builder()
            .status(500)
            .body(Body::from(error.clone()))
            .unwrap();
    }
    let param = [
        ("code", params.code.clone()),
        ("redirect_uri", format!("{}callback", app_state.app_url)),
        ("grant_type", "authorization_code".to_string()),
    ];
    println!("Sending req soon...!");
    let req = app_state
        .http_client
        .post("https://accounts.spotify.com/api/token")
        .form(&param)
        .header("Content-type", "application/x-www-form-urlencoded")
        .header(
            "Authorization",
            STANDARD.encode(format!(
                "{}:{}",
                app_state.client_id, app_state.client_secret
            )),
        )
        .send()
        .await;

    match req {
        Ok(val) => return format!("{}", val.text().await.unwrap()).into_response(),
        Err(e) => return format!("ERROR").into_response(),
    }
}
