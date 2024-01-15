use cgp_core::prelude::*;
use cgp_core::{CanRun, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use eyre::Error;
use hermes_async_runtime_components::task::types::future_task::FutureTask;
use hermes_cosmos_relayer::contexts::birelay::CosmosBiRelay;
use hermes_relayer_components_extra::runtime::traits::spawn::CanSpawnTask;
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
        ErrorTypeComponent:
            ProvideEyreError,
        ErrorRaiserComponent:
            RaiseDebugError,
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
