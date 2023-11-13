use cgp_core::Async;

pub trait HasAddressType: Async {
    type Address: Async;
}
