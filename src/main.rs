use axum::routing::get;
/// Use axum capabilities.
use dotenv::dotenv;
use std::env;
#[tokio::main]
pub async fn main() {
    // Build our application by creating our router.
    let app = axum::Router::new()
        .fallback(fallback)
        .route("/", get(hello))
        .route("/read-env", get(read_env));

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for "GET /" which returns a string and causes axum to
/// immediately respond with status code `200 OK` and with the string.
pub async fn hello() -> String {
    "Hello, World".to_string()
}

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

/// axum handler for "GET /read-env" which returns a HTTP status
/// code, such as OK (200), and a custom string read from the environment.
pub async fn read_env() -> (axum::http::StatusCode, String) {
    dotenv().ok();
    let text = env::var("TEXT").expect("TEXT must be set");
    (axum::http::StatusCode::OK, text)
}
