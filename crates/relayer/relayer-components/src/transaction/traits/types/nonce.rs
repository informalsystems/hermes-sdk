use hermes_prelude::*;

#[cgp_type]
pub trait HasNonceType: Async {
    type Nonce: Async;
}
