use crate::state::AppState;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::api::devices::Device;
use crate::api::error::Result;

#[derive(Debug, Serialize, Deserialize)]
struct Channel {
    id: i64,
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/list", get(list))
        .route("/create", post(create))
}

async fn list(State(state): State<AppState>) -> Result<Json<Vec<Channel>>> {
    let res = sqlx::query!("SELECT channelId as channel_id FROM CHANNEL ORDER BY channelId")
        .map(|c| Channel { id: c.channel_id })
        .fetch_all(state.conn())
        .await?;

    Ok(Json(res))
}

async fn create(State(state): State<AppState>) -> Result<Json<Channel>> {
    let conn = state.conn();

    let res = sqlx::query!(
        "INSERT INTO CHANNEL(channelId) VALUES(NULL) RETURNING channelId as channel_id"
    )
    .map(|c| Channel { id: c.channel_id })
    .fetch_one(conn)
    .await?;

    Ok(Json(res))
}

async fn subscribe(
    State(state): State<AppState>,
    channel_id: Path<i64>,
    device: Json<Device>,
) -> Result<()> {
    unimplemented!()
}
