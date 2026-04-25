use std::env;

use axum::{Router, extract::State, routing::get, serve};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let env = Env {
        DATABASE_URL: env::var("DATABASE_URL").unwrap(),
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState {
        db: PgPoolOptions::new()
            .max_connections(10)
            .min_connections(1)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .idle_timeout(std::time::Duration::from_secs(300))
            .max_lifetime(std::time::Duration::from_secs(1800))
            .test_before_acquire(true)
            .connect_lazy(&env.DATABASE_URL)
            .unwrap(),
        env,
    };

    tracing::info!("Connected to db");

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health))
        .route("/pools", get(pools))
        .with_state(state);

    let port = 3000;

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    tracing::info!("server running on {}", port);

    serve(listener, app).await.unwrap();
}

struct PoolJson {
    name: String,
    apy: f32,
}

#[derive(Clone)]
#[allow(non_snake_case)]
struct Env {
    DATABASE_URL: String,
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
    env: Env,
}

async fn pools(State(state): State<AppState>) -> &'static str {
    "Hello from pools!"
}

async fn root_handler() -> &'static str {
    "{\"msg\":\"working\"}"
}

async fn health() -> &'static str {
    "{}"
}
