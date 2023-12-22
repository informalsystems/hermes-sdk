use cgp_core::prelude::*;

#[async_trait]
pub trait HasBackgroundRelayer: Async {
    fn start_relayer_in_background(&self);

    fn stop_relayer(&self);

    fn background_relayer_running(&self) -> bool;
}
