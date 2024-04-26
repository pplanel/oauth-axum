use std::sync::Arc;

use axum::extract::Query;
use axum::Router;
use axum::{routing::get, Extension};
use oauth_axum::client::{OAuthClient, Provider};
use oauth_axum::memory_db::AxumState;

#[derive(Clone, serde::Deserialize)]
pub struct QueryAxumCallback {
    pub code: String,
    pub state: String,
}

#[tokio::main]
async fn main() {
    println!("Starting server...");

    let state = Arc::new(AxumState::new());
    let app = Router::new()
        .route("/", get(create_url))
        .route("/api/v1/discord/callback", get(callback))
        .layer(Extension(state.clone()));

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn get_client() -> OAuthClient {
    OAuthClient::new(
        Provider::Discord,
        "1233429621748531333".to_string(),
        "jz6LQgN7DY8jIbSVLu4ivZFkbAdWr_qa".to_string(),
        "http://localhost:3000/api/v1/discord/callback".to_string(),
    )
}

pub async fn create_url(Extension(state): Extension<Arc<AxumState>>) -> String {
    get_client()
        .set_memory_state(Arc::clone(&state))
        .generate_url(Vec::from(["email".to_string()]))
        .url_generated
        .unwrap_or_default()
}

pub async fn callback(
    Extension(state): Extension<Arc<AxumState>>,
    Query(queries): Query<QueryAxumCallback>,
) -> String {
    println!("{:?}", state.clone().get_all_items());
    get_client()
        .set_memory_state(Arc::clone(&state))
        .generate_token_memory(queries.code, queries.state)
        .await
}