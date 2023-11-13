use cgp_core::Async;

pub trait HasRelayer: Async {
    fn run_relayer_in_background(&self);

    fn stop_relayer(&self);
}
