use axum::{
    extract::State,
    response::{Redirect, Response},
};
use serde::Serialize;


use crate::{AppState, REQUIRED_USER_PERMS};
#[derive(Serialize)]
pub struct LoginQueryReqs {
    response_type: String,
    client_id: String,
    scope: String,
    state: String,
    redirect_uri: String,
}
pub async fn login(State(app_state): State<AppState>) -> Redirect {
    let params = LoginQueryReqs {
        response_type: String::from("code"),
        client_id: app_state.client_id,
        scope: REQUIRED_USER_PERMS.to_string(),
        redirect_uri: format!("{}callback", app_state.app_url),
        state: String::from("ABCDABCDABCDABCD"),
    };
    Redirect::to(
        format!(
            "https://accounts.spotify.com/authorize?{}",
            serde_urlencoded::to_string(params).unwrap()
        )
        .as_str(),
    )
}
