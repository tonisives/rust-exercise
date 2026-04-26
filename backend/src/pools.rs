use axum::{Json, extract::State};

use crate::app_state::AppState;
use serde::Serialize;

#[derive(Serialize)]
pub struct PoolJson {
    name: String,
    apy: f32,
}

pub async fn get_pools(State(state): State<AppState>) -> Json<PoolJson> {
    Json::from(PoolJson {
        name: "pool1".to_string(),
        apy: 0.2,
    })
}
