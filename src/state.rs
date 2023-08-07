use reqwest::Client;
use sqlx::SqlitePool;
use std::sync::Arc;

pub(crate) trait AppStateExt {
    fn new_state(conn: SqlitePool) -> Self;
}

pub(crate) type AppState = Arc<InnerState>;

impl AppStateExt for AppState {
    fn new_state(conn: SqlitePool) -> Self {
        let client = Client::new();
        Self::new(InnerState { conn, client })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct InnerState {
    conn: SqlitePool,
    client: Client,
}

impl InnerState {
    pub(crate) fn conn(&self) -> &SqlitePool {
        &self.conn
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }
}
