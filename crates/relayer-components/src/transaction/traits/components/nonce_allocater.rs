use cgp_core::{async_trait, derive_component, HasErrorType};

use crate::std_prelude::*;
use crate::transaction::traits::nonce::guard::HasNonceGuard;
use crate::transaction::traits::types::HasSignerType;

#[derive_component(NonceAllocatorComponent, NonceAllocator<TxContext>)]
#[async_trait]
pub trait CanAllocateNonce: HasNonceGuard + HasSignerType + HasErrorType {
    async fn allocate_nonce<'a>(
        &'a self,
        signer: &'a Self::Signer,
    ) -> Result<Self::NonceGuard<'a>, Self::Error>;
}
