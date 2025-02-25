use cgp::prelude::*;
use futures::lock::MutexGuard;

use crate::transaction::traits::types::nonce::HasNonceType;
use crate::transaction::traits::types::signer::HasSignerType;

#[cgp_component {
  provider: NonceAllocator,
  context: TxContext,
}]
#[async_trait]
pub trait CanAllocateNonce: HasNonceType + HasSignerType + HasAsyncErrorType {
    async fn allocate_nonce<'a>(
        &'a self,
        signer: &'a Self::Signer,
    ) -> Result<(MutexGuard<'a, ()>, Self::Nonce), Self::Error>;
}
