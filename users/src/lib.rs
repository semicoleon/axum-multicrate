use std::sync::Arc;

use axum::{extract::State, Router};
use interface::{ItemModule, UserModule};

pub mod data;

/// Create a router for the item service, which requires state containing the modules interface.
pub fn routes<T: ItemModule + UserModule>() -> Router<Arc<T>> {
    Router::new().route("/", axum::routing::get(get_users))
}

async fn get_users<T: ItemModule + UserModule>(State(modules): State<Arc<T>>) -> String {
    let mut conn = modules.pool().get().unwrap();

    tokio::task::spawn_blocking(move || format!("{:#?}", modules.load_users(&mut conn, &*modules)))
        .await
        .unwrap()
}
