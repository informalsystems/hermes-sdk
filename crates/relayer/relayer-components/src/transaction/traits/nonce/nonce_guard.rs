use core::ops::Deref;

use cgp::prelude::*;

use crate::transaction::traits::types::nonce::HasNonceType;

#[derive_component(NonceGuardComponent, ProvideNonceGuard<Chain>)]
pub trait HasNonceGuard: HasNonceType {
    type NonceGuard<'a>: Deref<Target = Self::Nonce> + Send + Sync;
}
