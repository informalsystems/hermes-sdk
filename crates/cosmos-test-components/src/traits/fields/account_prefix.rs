use cgp_core::Async;

pub trait HasAccountPrefix: Async {
    fn account_prefix(&self) -> &str;
}
