use cgp::prelude::*;
use futures::lock::MutexGuard;

use crate::transaction::traits::{
    CanQueryNonce, HasMutexForNonceAllocation, NonceAllocator, NonceAllocatorComponent,
};

pub struct AllocateNonceWithMutex;

#[cgp_provider(NonceAllocatorComponent)]
impl<Context> NonceAllocator<Context> for AllocateNonceWithMutex
where
    Context: CanQueryNonce + HasMutexForNonceAllocation,
{
    async fn allocate_nonce<'a>(
        context: &'a Context,
        signer: &'a Context::Signer,
    ) -> Result<(MutexGuard<'a, ()>, Context::Nonce), Context::Error> {
        let mutex = context.mutex_for_nonce_allocation(signer);

        let mutex_guard = mutex.lock().await;

        let nonce = context.query_nonce(signer).await?;

        Ok((mutex_guard, nonce))
    }
}
