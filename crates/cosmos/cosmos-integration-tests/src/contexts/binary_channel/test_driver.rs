use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_ibc_test_suite::traits::CanUseBinaryTestDriverMethods;
use hermes_logging_components::traits::logger::LoggerComponent;
use hermes_relayer_components::multi::traits::birelay_at::BiRelayTypeProviderAtComponent;
use hermes_relayer_components::multi::traits::chain_at::ChainTypeProviderAtComponent;
use hermes_relayer_components::multi::traits::relay_at::RelayTypeProviderAtComponent;
use hermes_test_components::driver::traits::channel_at::ChannelIdGetterAtComponent;
use hermes_test_components::driver::traits::types::chain_driver_at::{
    ChainDriverGetterAtComponent, ChainDriverTypeProviderAtComponent,
};
use hermes_test_components::driver::traits::types::relay_driver_at::{
    RelayDriverGetterAtComponent, RelayDriverTypeProviderAtComponent,
};
use hermes_test_components::setup::traits::port_id_at::PortIdGetterAtComponent;
use hermes_tracing_logging_components::contexts::logger::TracingLogger;
use ibc::core::host::types::identifiers::{ChannelId, ConnectionId, PortId};

use crate::contexts::chain_driver::CosmosChainDriver;
use crate::contexts::relay_driver::CosmosRelayDriver;
use crate::impls::test_driver::types::UseCosmosTestTypes;

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
