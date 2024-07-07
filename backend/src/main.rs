mod postgres;
use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing, Json, Router,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

use crate::postgres::{rest_router, AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&db_url).await?;

    let app = Router::new()
        .route("/", routing::get(handler))
        .merge(rest_router())
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_headers([CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]),
        )
        .with_state(AppState::new(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: format!("Hello, World!"),
    })
}
