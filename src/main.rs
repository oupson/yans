use std::{env, net::SocketAddr};

use anyhow::Context;
use axum::Router;
use sqlx::SqlitePool;

use crate::state::{AppState, AppStateExt};

pub(crate) mod api;
pub(crate) mod state;
pub(crate) mod unified_push;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    let conn = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    sqlx::migrate!("./migrations").run(&conn).await?;

    let state = AppState::new_state(conn);

    // build our application with a route
    let app = Router::new().merge(api::router()).with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("Failed to listen")?;

    Ok(())
}
