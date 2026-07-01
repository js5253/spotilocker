// login with username and password

use axum::{
    Json,
    extract::State,
    http::{Error, Result},
};
use scrypt::{
    password_hash::{
        PasswordHasher, PasswordVerifier, phc::{PasswordHash, Salt}
    },
};
use crate::{AppState, errors::AppError, models::User};

struct LoginRequest {
    username: String,
    password: String,
}
// #[axum::debug_handler]
pub fn app_login(app_state: State<AppState>, body: Json<LoginRequest>) -> Result<Json<String>> {
    // let users: Vec<User> = User::find().all(db).await?;

    // for user in users {
    //     println!("User email: {}", user.email);
    // }

    Ok(Json("".to_string()))
}

pub async fn app_register(app_state: State<AppState>, body: Json<LoginRequest>) -> axum::response::Result<(), AppError> {
    let scrypt = Scrypt::default(); // Uses `Params::RECOMMENDED`
    let hashed = scrypt.has
    let req = sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)").bind(body.username.clone()).bind(body.password.clone()).execute(&app_state.db).await?
    if req.rows_affected() != 0 {

    } else {

    }
    Ok(())
}