use core::marker::PhantomData;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_logger::ProvideHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::multi::traits::birelay_at::BiRelayTypeAtComponent;
use hermes_relayer_components::multi::traits::chain_at::ChainTypeAtComponent;
use hermes_relayer_components::multi::traits::relay_at::RelayTypeAtComponent;
use hermes_test_components::driver::traits::channel_at::ChannelGetterAt;
use hermes_test_components::driver::traits::types::chain_driver_at::{
    ChainDriverGetterAt, ChainDriverTypeAtComponent,
};
use hermes_test_components::driver::traits::types::relay_driver_at::{
    RelayDriverGetterAt, RelayDriverTypeAtComponent,
};
use ibc::core::host::types::identifiers::{ChannelId, ConnectionId, PortId};

use crate::contexts::chain_driver::CosmosChainDriver;
use crate::contexts::relay_driver::CosmosRelayDriver;
use crate::impls::test_driver::types::ProvideCosmosTestTypes;

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

pub struct CosmosBinaryChannelTestDriverComponents;

impl HasComponents for CosmosBinaryChannelTestDriver {
    type Components = CosmosBinaryChannelTestDriverComponents;
}

delegate_components! {
    CosmosBinaryChannelTestDriverComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DebugError,
        [
            ChainTypeAtComponent<Index<0>>,
            ChainTypeAtComponent<Index<1>>,
            ChainDriverTypeAtComponent,
            RelayTypeAtComponent<Index<0>, Index<1>>,
            RelayTypeAtComponent<Index<1>, Index<0>>,
            BiRelayTypeAtComponent<Index<0>, Index<1>>,
            RelayDriverTypeAtComponent,
        ]:
            ProvideCosmosTestTypes,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideHermesLogger,
    }
}

impl ChainDriverGetterAt<CosmosBinaryChannelTestDriver, Index<0>>
    for CosmosBinaryChannelTestDriverComponents
{
    fn chain_driver_at(
        driver: &CosmosBinaryChannelTestDriver,
        _index: PhantomData<Index<0>>,
    ) -> &CosmosChainDriver {
        &driver.chain_driver_a
    }
}

impl ChainDriverGetterAt<CosmosBinaryChannelTestDriver, Index<1>>
    for CosmosBinaryChannelTestDriverComponents
{
    fn chain_driver_at(
        driver: &CosmosBinaryChannelTestDriver,
        _index: PhantomData<Index<1>>,
    ) -> &CosmosChainDriver {
        &driver.chain_driver_b
    }
}

impl RelayDriverGetterAt<CosmosBinaryChannelTestDriver, Index<0>, Index<1>>
    for CosmosBinaryChannelTestDriverComponents
{
    fn relay_driver_at(
        driver: &CosmosBinaryChannelTestDriver,
        _index: PhantomData<(Index<0>, Index<1>)>,
    ) -> &CosmosRelayDriver {
        &driver.relay_driver
    }
}

impl ChannelGetterAt<CosmosBinaryChannelTestDriver, Index<0>, Index<1>>
    for CosmosBinaryChannelTestDriverComponents
{
    fn channel_id_at(
        driver: &CosmosBinaryChannelTestDriver,
        _index: PhantomData<(Index<0>, Index<1>)>,
    ) -> &ChannelId {
        &driver.channel_id_a
    }

    fn port_id_at(
        driver: &CosmosBinaryChannelTestDriver,
        _index: PhantomData<(Index<0>, Index<1>)>,
    ) -> &PortId {
        &driver.port_id_a
    }
}

impl ChannelGetterAt<CosmosBinaryChannelTestDriver, Index<1>, Index<0>>
    for CosmosBinaryChannelTestDriverComponents
{
    fn channel_id_at(
        driver: &CosmosBinaryChannelTestDriver,
        _index: PhantomData<(Index<1>, Index<0>)>,
    ) -> &ChannelId {
        &driver.channel_id_b
    }

    fn port_id_at(
        driver: &CosmosBinaryChannelTestDriver,
        _index: PhantomData<(Index<1>, Index<0>)>,
    ) -> &PortId {
        &driver.port_id_b
    }
}
