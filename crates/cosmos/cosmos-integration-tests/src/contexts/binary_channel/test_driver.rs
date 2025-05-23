use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::Index;
use hermes_core::logging_components::traits::LoggerComponent;
use hermes_core::relayer_components::multi::traits::birelay_at::BiRelayTypeProviderAtComponent;
use hermes_core::relayer_components::multi::traits::chain_at::ChainTypeProviderAtComponent;
use hermes_core::relayer_components::multi::traits::relay_at::RelayTypeProviderAtComponent;
use hermes_core::test_components::driver::traits::{
    ChainDriverGetterAtComponent, ChainDriverTypeProviderAtComponent, ChannelIdGetterAtComponent,
    RelayDriverGetterAtComponent, RelayDriverTypeProviderAtComponent,
};
use hermes_core::test_components::setup::traits::PortIdGetterAtComponent;
use hermes_cosmos_core::tracing_logging_components::contexts::TracingLogger;
use hermes_error::handlers::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_ibc_test_suite::traits::CanUseBinaryTestDriverMethods;
use hermes_prelude::*;
use ibc::core::host::types::identifiers::{ChannelId, ConnectionId, PortId};

use crate::contexts::{CosmosChainDriver, CosmosRelayDriver};
use crate::impls::UseCosmosTestTypes;

#[cgp_context(CosmosBinaryChannelTestDriverComponents)]
#[derive(HasField)]
pub struct CosmosBinaryChannelTestDriver {
    pub relay_driver: CosmosRelayDriver,
    pub chain_driver_a: CosmosChainDriver,
    pub chain_driver_b: CosmosChainDriver,
    pub connection_id_a: ConnectionId,
    pub connection_id_b: ConnectionId,
    pub channel_id_a: ChannelId,
    pub channel_id_b: ChannelId,
    pub port_id_a: PortId,
    pub port_id_b: PortId,
}

delegate_components! {
    CosmosBinaryChannelTestDriverComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        [
            ChainTypeProviderAtComponent<Index<0>>,
            ChainTypeProviderAtComponent<Index<1>>,
            ChainDriverTypeProviderAtComponent<Index<0>>,
            ChainDriverTypeProviderAtComponent<Index<1>>,
            RelayTypeProviderAtComponent<Index<0>, Index<1>>,
            RelayTypeProviderAtComponent<Index<1>, Index<0>>,
            BiRelayTypeProviderAtComponent<Index<0>, Index<1>>,
            RelayDriverTypeProviderAtComponent<Index<0>, Index<1>>,
        ]:
            UseCosmosTestTypes,
        LoggerComponent: TracingLogger,
        ChainDriverGetterAtComponent<Index<0>>:
            UseField<symbol!("chain_driver_a")>,
        ChainDriverGetterAtComponent<Index<1>>:
            UseField<symbol!("chain_driver_b")>,
        RelayDriverGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("relay_driver")>,
        ChannelIdGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("channel_id_a")>,
        ChannelIdGetterAtComponent<Index<1>, Index<0>>:
            UseField<symbol!("channel_id_b")>,
        PortIdGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("port_id_a")>,
        PortIdGetterAtComponent<Index<1>, Index<0>>:
            UseField<symbol!("port_id_b")>,
    }
}

pub trait CanUseCosmosTestDriver: CanUseBinaryTestDriverMethods<Index<0>, Index<1>> {}

impl CanUseCosmosTestDriver for CosmosBinaryChannelTestDriver {}
