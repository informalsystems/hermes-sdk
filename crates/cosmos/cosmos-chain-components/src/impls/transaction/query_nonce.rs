use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::nonce::query_nonce::{
    NonceQuerier, NonceQuerierComponent,
};
use hermes_relayer_components::transaction::traits::types::nonce::HasNonceType;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use http::uri::InvalidUri;
use prost::DecodeError;
use tonic::transport::Error as TransportError;
use tonic::Status;

use crate::traits::grpc_address::HasGrpcAddress;
use crate::types::key_types::secp256k1::Secp256k1KeyPair;
use crate::types::transaction::account::{query_account, Account};

pub struct QueryCosmosAccount;

#[cgp_provider(NonceQuerierComponent)]
impl<Chain> NonceQuerier<Chain> for QueryCosmosAccount
where
    Chain: HasSignerType<Signer = Secp256k1KeyPair>
        + HasNonceType<Nonce = Account>
        + HasGrpcAddress
        + CanRaiseAsyncError<DecodeError>
        + CanRaiseAsyncError<TransportError>
        + CanRaiseAsyncError<InvalidUri>
        + CanRaiseAsyncError<Status>
        + CanRaiseAsyncError<String>,
{
    async fn query_nonce(
        chain: &Chain,
        key_pair: &Secp256k1KeyPair,
    ) -> Result<Account, Chain::Error> {
        let address = key_pair.account();

        query_account(chain, address.to_string()).await
    }
}
