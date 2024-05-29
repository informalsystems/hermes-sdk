use cgp_core::prelude::HasErrorType;
use cgp_core::CanRaiseError;
use ed25519_dalek::SigningKey;
use hermes_relayer_components::transaction::traits::nonce::query_nonce::NonceQuerier;
use hermes_relayer_components::transaction::traits::types::nonce::HasNonceType;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::Deserialize;

use crate::traits::json_rpc_client::HasJsonRpcClient;

pub struct QuerySovereignNonce;

impl<Rollup> NonceQuerier<Rollup> for QuerySovereignNonce
where
    Rollup: HasSignerType<Signer = SigningKey>
        + HasNonceType<Nonce = u64>
        + HasErrorType
        + HasJsonRpcClient
        + CanRaiseError<ClientError>,
    Rollup::JsonRpcClient: ClientT,
{
    async fn query_nonce(rollup: &Rollup, signer: &SigningKey) -> Result<u64, Rollup::Error> {
        let public_key = signer.verifying_key();
        let key_bytes = public_key.as_bytes();

        let response: Response = rollup
            .json_rpc_client()
            .request("accounts_getAccount", (key_bytes,))
            .await
            .map_err(Rollup::raise_error)?;

        match response {
            Response::AccountExists { nonce } => Ok(nonce),
            Response::AccountEmpty => Ok(0),
        }
    }
}

#[derive(Deserialize)]
pub enum Response {
    AccountExists { nonce: u64 },
    AccountEmpty,
}
