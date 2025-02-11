use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::Index;
use cgp::extra::run::CanRun;
use cgp::prelude::*;
use hermes_async_runtime_components::task::types::future_task::FutureTask;
use hermes_cosmos_relayer::contexts::birelay::CosmosBiRelay;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_error::types::Error;
use hermes_relayer_components::multi::traits::birelay_at::{
    BiRelayTypeAtComponent, ProvideBiRelayTypeAt,
};
use hermes_relayer_components::multi::traits::chain_at::{
    ChainTypeAtComponent, ProvideChainTypeAt,
};
use hermes_relayer_components::multi::traits::relay_at::{
    ProvideRelayTypeAt, RelayTypeAtComponent,
};
use hermes_runtime_components::traits::spawn::CanSpawnTask;
use hermes_test_components::relay_driver::run::{
    RelayerBackgroundRunner, RelayerBackgroundRunnerComponent,
};

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

#[cgp_provider(RelayerBackgroundRunnerComponent)]
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

#[cgp_provider(ChainTypeAtComponent<Index<0>>)]
impl ProvideChainTypeAt<CosmosRelayDriver, Index<0>> for CosmosRelayDriverComponents {
    type Chain = CosmosChain;
}

#[cgp_provider(ChainTypeAtComponent<Index<1>>)]
impl ProvideChainTypeAt<CosmosRelayDriver, Index<1>> for CosmosRelayDriverComponents {
    type Chain = CosmosChain;
}

#[cgp_provider(RelayTypeAtComponent<Index<0>, Index<1>>)]
impl ProvideRelayTypeAt<CosmosRelayDriver, Index<0>, Index<1>> for CosmosRelayDriverComponents {
    type Relay = CosmosRelay;
}

#[cgp_provider(RelayTypeAtComponent<Index<1>, Index<0>>)]
impl ProvideRelayTypeAt<CosmosRelayDriver, Index<1>, Index<0>> for CosmosRelayDriverComponents {
    type Relay = CosmosRelay;
}

#[cgp_provider(BiRelayTypeAtComponent<Index<0>, Index<1>>)]
impl ProvideBiRelayTypeAt<CosmosRelayDriver, Index<0>, Index<1>> for CosmosRelayDriverComponents {
    type BiRelay = CosmosBiRelay;
}
