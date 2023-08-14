use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    routing::post,
    Json, Router, TypedHeader,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::error::{Error, Result},
    state::AppState,
};

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/unregister", post(unregister))
}

#[derive(Deserialize)]
struct RegisterBody {
    push_url: String,
}

#[derive(Serialize)]
pub(crate) struct Device {
    id: i64,
    push_url: String,
    token: String,
}

async fn register(
    State(state): State<AppState>,
    Json(register): Json<RegisterBody>,
) -> Result<Json<Device>> {
    let is_valid_push_url =
        crate::unified_push::validate_url(state.client(), &register.push_url).await;

    if is_valid_push_url {
        let res = sqlx::query!(
            "INSERT INTO REMOTE_DEVICE(remoteDeviceUrl) VALUES(?) RETURNING remoteDeviceId as id, remoteDeviceUrl as url, remoteDeviceToken as token",
        register.push_url
    )
    .map(|c| Device {
        id: c.id,
        push_url: c.url,
        token: c.token
    }).fetch_one(state.conn()).await?;
        Ok(Json(res))
    } else {
        Err(Error::InvalidPushUrl())
    }
}


async fn unregister(
    State(state): State<AppState>,
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
        sqlx::query!("DELETE FROM SUBSCRIBED WHERE remoteDeviceId = ?", remote_device_id).execute(state.conn()).await?;
        sqlx::query!("DELETE FROM REMOTE_DEVICE WHERE remoteDeviceId = ?", remote_device_id).execute(state.conn()).await?;
        Ok(())
    } else {
        Err(Error::DeviceNotFound())
    }
}
