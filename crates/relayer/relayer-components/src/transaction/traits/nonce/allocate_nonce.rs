use cgp_core::prelude::*;

use crate::transaction::traits::nonce::nonce_guard::HasNonceGuard;
use crate::transaction::traits::types::signer::HasSignerType;

#[derive_component(NonceAllocatorComponent, NonceAllocator<TxContext>)]
#[async_trait]
pub trait CanAllocateNonce: HasNonceGuard + HasSignerType + HasErrorType {
    async fn allocate_nonce<'a>(
        &'a self,
        signer: &'a Self::Signer,
    ) -> Result<Self::NonceGuard<'a>, Self::Error>;
}
