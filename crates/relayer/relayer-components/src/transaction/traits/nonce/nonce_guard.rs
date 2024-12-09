use core::ops::Deref;

use cgp::prelude::*;

use crate::transaction::traits::types::nonce::HasNonceType;

#[cgp_component {
  name: NonceGuardComponent,
  provider: ProvideNonceGuard,
  context: Chain,
}]
pub trait HasNonceGuard: HasNonceType {
    type NonceGuard<'a>: Deref<Target = Self::Nonce> + Send + Sync;
}
