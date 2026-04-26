use axum::{Router, extract::State, routing::get, serve};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app_state;
use app_state::AppState;

mod pools;
use pools::get_pools;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = app_state::init();
    tracing::info!("Connected to db");

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health))
        .route("/pools", get(get_pools))
        .with_state(state);

    let port = 3000;

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    tracing::info!("server running on {}", port);

    serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "{\"msg\":\"working\"}"
}

async fn health() -> &'static str {
    "{}"
}
