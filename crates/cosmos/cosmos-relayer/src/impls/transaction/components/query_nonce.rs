use cgp_core::prelude::*;
use hermes_relayer_components::transaction::traits::components::nonce_querier::NonceQuerier;
use ibc_relayer::chain::cosmos::query::account::query_account;
use ibc_relayer::chain::cosmos::types::account::Account;
use ibc_relayer::keyring::{Secp256k1KeyPair, SigningKeyPair};

use crate::contexts::transaction::CosmosTxContext;
use crate::impls::transaction::component::CosmosTxComponents;
use crate::types::error::Error;

#[async_trait]
impl NonceQuerier<CosmosTxContext> for CosmosTxComponents {
    async fn query_nonce(
        context: &CosmosTxContext,
        key_pair: &Secp256k1KeyPair,
    ) -> Result<Account, Error> {
        let tx_config = &context.tx_config;
        let address = key_pair.account();

        let account = query_account(&tx_config.grpc_address, &address).await?;

        Ok(account.into())
    }
}
