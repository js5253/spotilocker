mod models;
mod routes;
mod api;
mod errors;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use reqwest;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use std::env;

use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

const REQUIRED_USER_PERMS: &str = "user-read-private user-read-email user-library-read user-follow-read playlist-read-private";

const ADDR_TO_BIND: &str = "0.0.0.0:3000";

#[derive(Clone)]
struct AppState {
    client_id: String,
    client_secret: String,
    app_url: String,
    http_client: reqwest::Client,
    db: Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("spotilocker_2=error,tower_http=warn"))
                .unwrap(),
        )
        .init();
    // build our application with a single route
    dotenv().expect("Missing .env - copy .env.example and fill out.");
    let pool = SqlitePoolOptions::new().max_connections(5).connect(env::var("DATABASE_URL").expect("Could not find SQLite Database URL").as_str()).await.expect("Could not connect to database.");

    let state = AppState {
        client_id: env::var("CLIENT_ID").expect("Missing Client ID on .env"),
        app_url: env::var("SERVER_URL").expect("Missing Server URL on .env"),
        client_secret: env::var("CLIENT_SECRET").expect("Missing Client Secret on .env"),
        http_client: reqwest::Client::new(),
        db: pool
        // db: Database::connect(.expect("Missing DB URL"))
        //     .await
        //     .expect("Could not connect to database"),
    };
    let app = Router::new()
        .route(
            "/",
            get(|| async {
                // let pear = Track::ActiveModel {
                //     name: Set("Pear".to_owned()),
                //     ..Default::default() // all other attributes are `NotSet`
                // };

                // let pear: Track::Model = pear.insert(state.db).await;
            }),
        )
        // .route("/login", get(routes::app_login))
        .route("/spotify_login", get(routes::login))
        .route("/callback", get(routes::login_callback))
        .route("/top_songs", get(routes::top_songs))
        .route("/library", get(routes::refresh_library))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(ADDR_TO_BIND).await.unwrap();
    println!("listening on {}", ADDR_TO_BIND);
    axum::serve(listener, app).await.unwrap();
}
