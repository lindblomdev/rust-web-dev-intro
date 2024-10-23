use axum::extract::FromRef;
use sqlx::PgPool;

use super::{jwt::JWTKeyProvider, store::Store};

#[derive(Clone, FromRef)]
pub(crate) struct AppState {
    pub store: Store,
    pub jwt_key_provider: JWTKeyProvider,
}

impl AppState {
    pub fn new(pool: PgPool, jwt_base64: &str) -> Self {
        Self {
            store: Store::new(pool),
            jwt_key_provider: JWTKeyProvider::new(jwt_base64),
        }
    }
}
