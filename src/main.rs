mod routes;
use axum::{
    Router,
    body::Body,
    http::{Request, Response, StatusCode, Uri},
    routing::get,
};
use dotenvy::dotenv;
use reqwest;
use std::env;
use tower::ServiceExt;
use tower_http::services::ServeDir;
const ADDR_TO_BIND: &str = "0.0.0.0:3000";
#[derive(Default, Clone)]
struct AppState {
    client_id: String,
    client_secret: String,
    app_url: String,
    http_client: reqwest::Client,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    dotenv().expect("Missing .env");
    let state = AppState {
        client_id: env::var("CLIENT_ID").expect("Missing Client ID on .env"),
        app_url: env::var("SERVER_URL").expect("Missing Server URL on .env"),
        client_secret: env::var("CLIENT_SECRET").expect("Missing Client Secret on .env"),
        http_client: reqwest::Client::new(),
    };
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/login", get(routes::login))
        .route("/callback", get(routes::login_callback))
        .route("/library", get(routes::refresh_library))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(ADDR_TO_BIND).await.unwrap();
    println!("listening on {}", ADDR_TO_BIND);
    axum::serve(listener, app).await.unwrap();
}
