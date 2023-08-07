use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{api::error::{Result, Error}, state::AppState};

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/register", post(register))
}

#[derive(Deserialize)]
struct RegisterBody {
    push_url: String,
}

#[derive(Serialize)]
pub(crate) struct Device {
    id: i64,
    push_url: String,
}

async fn register(
    State(state): State<AppState>,
    Json(register): Json<RegisterBody>,
) -> Result<Json<Device>> {
    let is_valid_push_url =
        crate::unified_push::validate_url(state.client(), &register.push_url).await;

    if is_valid_push_url {
        let res = sqlx::query!(
        "INSERT INTO REMOTE_DEVICE(remoteDeviceUrl) VALUES(?) RETURNING remoteDeviceId as id, remoteDeviceUrl as url",
        register.push_url
    )
    .map(|c| Device {
        id: c.id,
        push_url: c.url,
    }).fetch_one(state.conn()).await?;

        Ok(Json(res))
    } else {
        Err(Error::InvalidPushUrl())
    }
}
