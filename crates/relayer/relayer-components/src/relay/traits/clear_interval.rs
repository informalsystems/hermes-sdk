use cgp::core::Async;

pub trait HasClearInterval {
    type ClearInterval: Async + Clone + Into<u64>;

    fn clear_interval(&self) -> Self::ClearInterval;
}
