use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Router,
};
use interface::{ItemModule, UserModule};

pub mod data;

/// Create a router for the item service, which requires state containing the modules interface.
pub fn routes<T: ItemModule + UserModule>() -> Router<Arc<T>> {
    Router::new().route("/user/:user_id", axum::routing::get(get_items))
}

async fn get_items<T: ItemModule + UserModule>(
    State(modules): State<Arc<T>>,
    Path(user_id): Path<i64>,
) -> String {
    let mut conn = modules.pool().get().unwrap();

    tokio::task::spawn_blocking(move || {
        format!("{:#?}", modules.load_items_user_id(&mut conn, user_id))
    })
    .await
    .unwrap()
}
