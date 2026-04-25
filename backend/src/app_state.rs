use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

#[derive(Clone)]
#[allow(non_snake_case)]
struct Env {
    DATABASE_URL: String,
}

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    env: Env,
}

pub fn init() -> AppState {
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

    state
}
