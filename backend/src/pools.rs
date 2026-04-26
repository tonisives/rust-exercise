use axum::{Json, extract::State};
use sqlx::{prelude::FromRow, query_as, types::JsonValue};

use crate::app_state::AppState;
use serde::Serialize;

#[derive(Serialize, FromRow)]
pub struct PoolJson {
    pool_name: String,
    apy: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    underlying_tokens: Option<JsonValue>,
}

pub async fn get_pools(State(state): State<AppState>) -> Json<Vec<PoolJson>> {
    let pools: Vec<PoolJson> = query_as(
        r#"
            SELECT pool_name, apy::float8 as apy, underlying_tokens FROM yield_pools limit 10;
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("query failed {e:?}");
        e
    })
    .unwrap_or_default();

    Json(pools)
}
