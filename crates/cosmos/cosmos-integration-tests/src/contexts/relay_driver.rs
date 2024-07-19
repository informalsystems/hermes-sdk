use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use cgp_core::run::CanRun;
use hermes_async_runtime_components::task::types::future_task::FutureTask;
use hermes_cosmos_relayer::contexts::birelay::CosmosBiRelay;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_error::types::Error;
use hermes_relayer_components::multi::traits::birelay_at::ProvideBiRelayTypeAt;
use hermes_relayer_components::multi::traits::chain_at::ProvideChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::ProvideRelayTypeAt;
use hermes_runtime_components::traits::spawn::CanSpawnTask;
use hermes_test_components::relay_driver::run::RelayerBackgroundRunner;

pub struct CosmosRelayDriver {
    pub birelay: CosmosBiRelay,
}

pub struct CosmosRelayDriverComponents;

impl HasComponents for CosmosRelayDriver {
    type Components = CosmosRelayDriverComponents;
}

delegate_components! {
    CosmosRelayDriverComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DebugError,
    }
}

impl RelayerBackgroundRunner<CosmosRelayDriver> for CosmosRelayDriverComponents {
    type RunHandle<'a> = ();

    async fn run_relayer_in_background(relay_driver: &CosmosRelayDriver) -> Result<(), Error> {
        let birelay = relay_driver.birelay.clone();
        let runtime = &relay_driver.birelay.runtime;

        runtime.spawn_task(FutureTask::new(async move {
            let _ = birelay.run().await;
        }));

        Ok(())
    }
}

impl ProvideChainTypeAt<CosmosRelayDriver, 0> for CosmosRelayDriverComponents {
    type Chain = CosmosChain;
}

impl ProvideChainTypeAt<CosmosRelayDriver, 1> for CosmosRelayDriverComponents {
    type Chain = CosmosChain;
}

impl ProvideRelayTypeAt<CosmosRelayDriver, 0, 1> for CosmosRelayDriverComponents {
    type Relay = CosmosRelay;
}

impl ProvideRelayTypeAt<CosmosRelayDriver, 1, 0> for CosmosRelayDriverComponents {
    type Relay = CosmosRelay;
}

impl ProvideBiRelayTypeAt<CosmosRelayDriver, 0, 1> for CosmosRelayDriverComponents {
    type BiRelay = CosmosBiRelay;
}
