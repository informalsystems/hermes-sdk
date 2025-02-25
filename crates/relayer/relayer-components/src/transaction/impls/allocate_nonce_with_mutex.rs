use cgp::prelude::*;
use futures::lock::MutexGuard;

use crate::transaction::traits::nonce::allocate_nonce::{NonceAllocator, NonceAllocatorComponent};
use crate::transaction::traits::nonce::nonce_mutex::HasMutexForNonceAllocation;
use crate::transaction::traits::nonce::query_nonce::CanQueryNonce;

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
