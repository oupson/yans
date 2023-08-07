use crate::state::AppState;
use axum::Router;

pub(crate) mod channels;
pub(crate) mod devices;
pub(crate) mod error;

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .nest("/channels", channels::router())
        .nest("/devices", devices::router())
}
