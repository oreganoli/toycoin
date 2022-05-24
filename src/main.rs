pub mod block;
mod web;
use axum::{routing::*, Extension};
use web::*;

use crate::block::Blockchain;

#[tokio::main]
async fn main() {
    let blockchain = Blockchain::new();
    let router = axum::Router::new()
        .route("/pending", get(get_pending))
        .route("/balances", get(get_balances))
        .route("/chain", get(get_chain))
        .route("/commit", post(post_commit))
        .route("/guess", post(post_guess))
        .route("/wire", post(post_wire))
        .layer(Extension(State::from(blockchain)));
    if let Err(e) = axum::Server::bind(&"0.0.0.0:8083".parse().unwrap())
        .serve(router.into_make_service())
        .await
    {
        eprintln!("Error serving: {}", e)
    }
}
