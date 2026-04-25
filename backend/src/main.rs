use std::env;

use axum::{Router, routing::get, serve};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let env = Env {
        DATABASE_URL: env::var("DATABASE_URL").unwrap(),
    };

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

#[allow(non_snake_case)]
struct Env {
    DATABASE_URL: String,
}

struct AppState {
    db: PgPool,
    env: Env,
}

async fn pools() -> &'static str {
    "Hello from Axum!"
}

async fn root_handler() -> &'static str {
    "{\"msg\":\"working\"}"
}

async fn health() -> &'static str {
    "{}"
}
