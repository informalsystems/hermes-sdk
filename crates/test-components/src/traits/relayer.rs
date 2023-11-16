use cgp_core::Async;

pub trait HasBackgroundRelayer: Async {
    fn start_relayer_in_background(&self);

    fn stop_relayer(&self);
}
