use hermes_prelude::*;

#[cgp_type]
pub trait HasSignerType: Async {
    type Signer: Async;
}
