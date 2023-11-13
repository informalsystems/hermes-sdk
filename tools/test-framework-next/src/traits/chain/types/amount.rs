use cgp_core::Async;

pub trait HasAmountType: Async {
    type Amount: Eq + Ord + Async;
}
