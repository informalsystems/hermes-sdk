use cgp_core::prelude::HasErrorType;
use cgp_core::CanRaiseError;
use ed25519_dalek::{SigningKey, VerifyingKey};
use hermes_relayer_components::transaction::traits::nonce::query_nonce::NonceQuerier;
use hermes_relayer_components::transaction::traits::types::nonce::HasNonceType;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::Deserialize;
use sha2::{Digest, Sha256};

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
        let key_bytes = public_key_to_hash_bytes(&signer.verifying_key());

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

#[derive(Debug, Deserialize)]
pub enum Response {
    AccountExists { nonce: u64 },
    AccountEmpty,
}

pub fn public_key_to_hash_bytes(public_key: &VerifyingKey) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(public_key);
    hasher.finalize().into()
}
