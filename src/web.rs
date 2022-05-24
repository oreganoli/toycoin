use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::block::{Blockchain, BlockchainError};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
/// Request types for HTTP methods that get data from the request body.
mod requests;
pub use requests::*;

impl IntoResponse for BlockchainError {
    fn into_response(self) -> Response {
        let (status_code, error_text) = match self {
            BlockchainError::NegativeBalances(balances) => {
                let mut text = format!(
                    "In the current state, the following accounts would have negative balances:\n"
                );
                for (acc, bal) in balances {
                    text.push_str(&format!("- {} would be left with {} Toycoins\n", acc, bal));
                }
                // remove the final newline
                text = text.trim_end().to_owned();
                (StatusCode::UNPROCESSABLE_ENTITY, text)
            }
            BlockchainError::NegativeTransfer { from, to, amount } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!(
                    "{} attempted to steal {} Toycoins from {}!",
                    from, amount, to
                ),
            ),
        };
        let mut error_map = HashMap::new();
        error_map.insert("error", error_text);
        (status_code, Json(error_map)).into_response()
    }
}
/// Mutable application state to be shared between HTTP requests.
#[derive(Clone)]
pub struct State {
    blockchain: Arc<Mutex<Blockchain>>,
}
impl From<Blockchain> for State {
    fn from(bc: Blockchain) -> Self {
        Self {
            blockchain: Arc::new(Mutex::new(bc)),
        }
    }
}
/// GET /pending
pub async fn get_pending(Extension(state): Extension<State>) -> impl IntoResponse {
    let pending = state.blockchain.lock().unwrap().pending();
    Json(pending)
}
/// GET /balances
pub async fn get_balances(Extension(state): Extension<State>) -> impl IntoResponse {
    let balances = state.blockchain.lock().unwrap().balance();
    Json(balances)
}
/// GET /chain
pub async fn get_chain(Extension(state): Extension<State>) -> impl IntoResponse {
    let chain = state.blockchain.lock().unwrap().chain();
    Json(chain)
}
/// POST /commit
pub async fn post_commit(Extension(state): Extension<State>) -> impl IntoResponse {
    Json(state.blockchain.lock().unwrap().commit())
}
/// POST /guess
pub async fn post_guess(
    Extension(state): Extension<State>,
    Json(req): Json<GuessRequest>,
) -> impl IntoResponse {
    let correct = state.blockchain.lock().unwrap().guess(req.guess, req.miner);
    Json(correct)
}
/// POST /wire
pub async fn post_wire(
    Extension(state): Extension<State>,
    Json(req): Json<WireRequest>,
) -> impl IntoResponse {
    match state
        .blockchain
        .lock()
        .unwrap()
        .wire(req.from, req.to, req.amount)
    {
        Ok(_) => Ok(StatusCode::ACCEPTED),
        Err(e) => Err(e),
    }
}
