mod routes;
use axum::{
    Router,
    body::Body,
    http::{Request, Response, StatusCode, Uri},
    routing::get,
};
use dotenvy::dotenv;
use std::env;
use tower::ServiceExt;
use tower_http::services::ServeDir;
#[derive(Default, Clone)]
struct AppState {
    redirect_uri: String,
    client_id: String,
    app_url: String,
}
async fn get_static_file(uri: Uri) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    match ServeDir::new("assets/").oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}
#[tokio::main]
async fn main() {
    // build our application with a single route
    dotenv().ok();
    let state = AppState {
        redirect_uri: env::var("REDIRECT_URL").expect("Missing Redirect URL on .env"),
        client_id: env::var("CLIENT_ID").expect("Missing Client ID on .env"),
        app_url: env::var("SERVER_URL").expect("Missing Server URL on .env"),
    };
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/login", get(routes::login))
        .route("/callback", get(routes::login_callback))
        .route("/library", get(routes::refresh_library))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
