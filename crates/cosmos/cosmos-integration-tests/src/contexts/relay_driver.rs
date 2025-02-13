use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::{Index, WithField};
use cgp::core::types::WithType;
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
    BiRelayGetterAtComponent, BiRelayTypeAtComponent,
};
use hermes_relayer_components::multi::traits::chain_at::ChainTypeAtComponent;
use hermes_relayer_components::multi::traits::relay_at::RelayTypeAtComponent;
use hermes_runtime_components::traits::spawn::CanSpawnTask;
use hermes_test_components::relay_driver::run::{
    RelayerBackgroundRunner, RelayerBackgroundRunnerComponent,
};

#[derive(HasField)]
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
        [
            ChainTypeAtComponent<Index<0>>,
            ChainTypeAtComponent<Index<1>>,
        ]:
            WithType<CosmosChain>,
        [
            RelayTypeAtComponent<Index<0>, Index<1>>,
            RelayTypeAtComponent<Index<1>, Index<0>>,
        ]: WithType<CosmosRelay>,
        BiRelayTypeAtComponent<Index<0>, Index<1>>:
            WithType<CosmosBiRelay>,
        BiRelayGetterAtComponent<Index<0>, Index<1>>:
            WithField<symbol!("birelay")>,
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

pub trait CanUseCosmosRelayDriver:
    CanUseComponent<BiRelayTypeAtComponent<Index<0>, Index<1>>, (Index<0>, Index<1>)>
    + CanUseComponent<BiRelayGetterAtComponent<Index<0>, Index<1>>, (Index<0>, Index<1>)>
{
}

impl CanUseCosmosRelayDriver for CosmosRelayDriver {}
