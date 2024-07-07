use axum::extract::FromRef;
use sqlx::PgPool;

use super::store::Store;

#[derive(Clone, FromRef)]
pub(crate) struct AppState {
    store: Store,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            store: Store::new(pool),
        }
    }
}
