use cgp_core::CanRaiseError;
use hermes_relayer_components::transaction::traits::components::nonce_querier::NonceQuerier;
use hermes_relayer_components::transaction::traits::types::{HasNonceType, HasSignerType};
use ibc_relayer::chain::cosmos::query::account::query_account;
use ibc_relayer::chain::cosmos::types::account::Account;
use ibc_relayer::error::Error as RelayerError;
use ibc_relayer::keyring::{Secp256k1KeyPair, SigningKeyPair};

use crate::traits::grpc_address::HasGrpcAddress;

pub struct QueryCosmosAccount;

impl<Chain> NonceQuerier<Chain> for QueryCosmosAccount
where
    Chain: HasSignerType<Signer = Secp256k1KeyPair>
        + HasNonceType<Nonce = Account>
        + HasGrpcAddress
        + CanRaiseError<RelayerError>,
{
    async fn query_nonce(
        chain: &Chain,
        key_pair: &Secp256k1KeyPair,
    ) -> Result<Account, Chain::Error> {
        let address = key_pair.account();

        let account = query_account(chain.grpc_address(), &address)
            .await
            .map_err(Chain::raise_error)?;

        Ok(account.into())
    }
}
