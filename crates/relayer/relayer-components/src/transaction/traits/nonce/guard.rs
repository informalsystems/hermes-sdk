use core::ops::Deref;

use cgp_core::prelude::*;

use crate::transaction::traits::types::HasNonceType;

#[derive_component(NonceGuardComponent, ProvideNonceGuard<Chain>)]
pub trait HasNonceGuard: HasNonceType {
    type NonceGuard<'a>: Deref<Target = Self::Nonce> + Send + Sync;
}
