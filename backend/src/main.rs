use axum::{Router, routing::get, serve};
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health))
        .route("/pools", get(pools));

    let port = 3000;

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    println!("server running on {}", port);

    serve(listener, app).await.unwrap();
}

struct PoolJson {
    name: String,
    apy: f32,
}

struct AppState {
    db: PgPool,
}

async fn pools() -> &'static str {
    "Hello from Axum!"
}

async fn root_handler() -> &'static str {
    "Hello from Axum!"
}

async fn health() -> &'static str {
    "{}"
}
