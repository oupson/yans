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
    State(_state): State<AppState>,
    auth: TypedHeader<Authorization<Bearer>>,
) -> Result<()> {
    unimplemented!()
}
