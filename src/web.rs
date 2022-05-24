use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::block::{Blockchain, BlockchainError};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};

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
pub struct State {
    blockchain: Arc<Mutex<Blockchain>>,
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
pub async fn post_guess(Extension(state): Extension<State>) -> impl IntoResponse {
    todo!();
}
/// POST /wire
pub async fn post_wire(Extension(state): Extension<State>) -> impl IntoResponse {
    todo!();
}
