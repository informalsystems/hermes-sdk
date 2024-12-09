use cgp::prelude::*;

use crate::transaction::traits::types::nonce::HasNonceType;
use crate::transaction::traits::types::signer::HasSignerType;

#[cgp_component {
  provider: NonceQuerier,
  context: TxContext,
}]
#[async_trait]
pub trait CanQueryNonce: HasSignerType + HasNonceType + HasErrorType {
    async fn query_nonce(&self, signer: &Self::Signer) -> Result<Self::Nonce, Self::Error>;
}
