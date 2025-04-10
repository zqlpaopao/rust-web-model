use axum::routing::get;
use axum::Router;
pub fn route() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/ping", get(ping))
}

pub async fn ping() -> &'static str {
    "Pong!"
}

pub async fn root() -> &'static str {
    "Hello, World!"
}
