use crate::transaction::traits::nonce::allocate_nonce::NonceAllocator;
use crate::transaction::traits::nonce::nonce_mutex::HasMutexForNonceAllocation;
use crate::transaction::traits::nonce::query_nonce::CanQueryNonce;
use hermes_runtime_components::traits::mutex::HasMutex;

pub struct AllocateNonceWithMutex;

impl<Context> NonceAllocator<Context> for AllocateNonceWithMutex
where
    Context: CanQueryNonce + HasMutexForNonceAllocation,
    Context::Runtime: HasMutex,
{
    async fn allocate_nonce<'a>(
        context: &'a Context,
        signer: &'a Context::Signer,
    ) -> Result<Context::NonceGuard<'a>, Context::Error> {
        let mutex = context.mutex_for_nonce_allocation(signer);

        let mutex_guard = Context::Runtime::acquire_mutex(mutex).await;

        let nonce = context.query_nonce(signer).await?;

        let nonce_guard = Context::mutex_to_nonce_guard(mutex_guard, nonce);

        Ok(nonce_guard)
    }
}
