use futures::lock::MutexGuard;
use hermes_prelude::*;

use crate::transaction::traits::{HasNonceType, HasSignerType};

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
