use cgp::prelude::*;

#[cgp_type]
pub trait HasSignerType: Async {
    type Signer: Async;
}
