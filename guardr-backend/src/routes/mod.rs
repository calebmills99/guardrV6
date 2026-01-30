use axum::Router;

pub fn api_routes() -> Router {
    Router::new().route("/", axum::routing::get(|| async { "Guardr API" }))
}
