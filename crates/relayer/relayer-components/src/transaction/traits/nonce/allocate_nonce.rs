use cgp::prelude::*;

use crate::transaction::traits::nonce::nonce_guard::HasNonceGuard;
use crate::transaction::traits::types::signer::HasSignerType;

#[cgp_component {
  provider: NonceAllocator,
  context: TxContext,
}]
#[async_trait]
pub trait CanAllocateNonce: HasNonceGuard + HasSignerType + HasAsyncErrorType {
    async fn allocate_nonce<'a>(
        &'a self,
        signer: &'a Self::Signer,
    ) -> Result<Self::NonceGuard<'a>, Self::Error>;
}
