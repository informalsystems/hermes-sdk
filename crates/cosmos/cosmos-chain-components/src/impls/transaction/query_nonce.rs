use hermes_core::relayer_components::transaction::traits::{
    HasNonceType, HasSignerType, NonceQuerier, NonceQuerierComponent,
};
use hermes_prelude::*;
use http::uri::InvalidUri;
use prost::DecodeError;
use tonic::transport::Error as TransportError;
use tonic::Status;

use crate::traits::HasGrpcAddress;
use crate::types::{query_account, Account, Secp256k1KeyPair};

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
