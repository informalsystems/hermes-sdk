use cgp_core::Async;

pub trait HasDenomType: Async {
    type Denom: Async;
}
