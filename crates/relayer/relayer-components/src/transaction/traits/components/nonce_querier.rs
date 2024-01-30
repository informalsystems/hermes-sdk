use cgp_core::prelude::*;

use crate::transaction::traits::types::{HasNonceType, HasSignerType};

#[derive_component(NonceQuerierComponent, NonceQuerier<TxContext>)]
#[async_trait]
pub trait CanQueryNonce: HasSignerType + HasNonceType + HasErrorType {
    async fn query_nonce(&self, signer: &Self::Signer) -> Result<Self::Nonce, Self::Error>;
}
