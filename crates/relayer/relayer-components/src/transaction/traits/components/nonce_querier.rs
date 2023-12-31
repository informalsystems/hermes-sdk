use cgp_core::prelude::*;

use crate::transaction::traits::types::HasTxTypes;

#[derive_component(NonceQuerierComponent, NonceQuerier<TxContext>)]
#[async_trait]
pub trait CanQueryNonce: HasTxTypes {
    async fn query_nonce(&self, signer: &Self::Signer) -> Result<Self::Nonce, Self::Error>;
}
