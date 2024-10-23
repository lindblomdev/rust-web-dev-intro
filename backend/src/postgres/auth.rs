use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use jwt_simple::prelude::{ECDSAP256PublicKeyLike, NoCustomClaims};

use super::AppState;

pub struct RequireAuthentication(pub User);

#[derive(Clone, Debug)]
pub struct User {
    pub id: i32,
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for RequireAuthentication
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));

        if auth_header.is_some() {
            let app_state = AppState::from_ref(state);
            let claims = app_state
                .jwt_key_provider
                .key_pair
                .public_key()
                .verify_token::<NoCustomClaims>(auth_header.unwrap(), None)
                .unwrap();
            let id: i32 = claims.subject.unwrap().parse().unwrap();
            Ok(Self(User { id }))
        } else {
            Err(())
        }
    }
}
