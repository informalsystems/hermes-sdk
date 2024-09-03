use cgp::prelude::*;

#[derive_component(RelayerBackgroundRunnerComponent, RelayerBackgroundRunner<RelayDriver>)]
#[async_trait]
pub trait CanRunRelayerInBackground: Async + HasErrorType {
    type RunHandle<'a>: Send + Sync;

    /**
       Spawns the relayer as an async task to run in the background.
       This method should only be called once during test, or it may
       result in error.

       The background will keep running as long as the returned `RunHandle`
       is kept alive. If the relayer stops before the `RunHandle` is dropped,
       the relay driver may panic to cause the running test to fail.
    */
    async fn run_relayer_in_background(&self) -> Result<Self::RunHandle<'_>, Self::Error>;
}
