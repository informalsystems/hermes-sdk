use hermes_prelude::*;

use crate::transaction::traits::{HasNonceType, HasSignerType};

#[cgp_component {
  provider: NonceQuerier,
  context: TxContext,
}]
#[async_trait]
pub trait CanQueryNonce: HasSignerType + HasNonceType + HasAsyncErrorType {
    async fn query_nonce(&self, signer: &Self::Signer) -> Result<Self::Nonce, Self::Error>;
}
