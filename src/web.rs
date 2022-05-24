use std::collections::HashMap;

use crate::block::{self, BlockchainError};
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
