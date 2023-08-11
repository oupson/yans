use crate::state::AppState;
use axum::extract::{Path, State};
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::routing::{get, post};
use axum::{Json, Router, TypedHeader};
use serde::{Deserialize, Serialize};

use crate::api::error::{Error, Result};

#[derive(Debug, Serialize, Deserialize)]
struct Channel {
    id: i64,
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/list", get(list))
        .route("/create", post(create))
        .route("/subscribe/:channel_id", post(subscribe))
        .route("/send/:channel_id", post(send))
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
    auth: TypedHeader<Authorization<Bearer>>,
) -> Result<()> {
    let token = auth.token();
    let remote_device_id = sqlx::query!(
        "SELECT remoteDeviceId as device_id FROM REMOTE_DEVICE WHERE remoteDeviceToken = ?",
        token
    )
    .map(|r| r.device_id)
    .fetch_optional(state.conn())
    .await?;

    if let Some(remote_device_id) = remote_device_id {
        sqlx::query!(
            "INSERT INTO SUBSCRIBED(remoteDeviceId, channelId) VALUES(?, ?)",
            remote_device_id,
            *channel_id
        )
        .execute(state.conn())
        .await?;
        Ok(())
    } else {
        Err(Error::DeviceNotFound())
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct SendBody {
    title: String,
    content: String,
}

async fn send(State(state): State<AppState>, channel_id: Path<i64>) -> Result<()> {
    Ok(())
}
