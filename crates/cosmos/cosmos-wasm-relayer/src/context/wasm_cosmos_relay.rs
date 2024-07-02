use std::collections::BTreeSet;
use std::sync::Arc;

use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use cgp_core::run::CanRun;
use futures::lock::Mutex;
use hermes_cosmos_relayer::contexts::logger::{CosmosLogger, ProvideCosmosLogger};
use hermes_cosmos_relayer::impls::error::HandleCosmosError;
use hermes_cosmos_relayer::types::error::Error;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_logging_components::traits::logger::CanLog;
use hermes_relayer_components::chain::traits::types::channel::HasInitChannelOptionsType;
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;
use hermes_relayer_components::components::default::relay::*;
use hermes_relayer_components::error::impls::retry::ReturnMaxRetry;
use hermes_relayer_components::error::traits::retry::{
    MaxErrorRetryGetterComponent, RetryableErrorComponent,
};
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::impls::packet_lock::{
    PacketMutexGetter, ProvidePacketLockWithMutex,
};
use hermes_relayer_components::relay::impls::packet_relayers::general::lock::LogSkipRelayLockedPacket;
use hermes_relayer_components::relay::traits::chains::ProvideRelayChains;
use hermes_relayer_components::relay::traits::packet_filter::PacketFilter;
use hermes_relayer_components::relay::traits::packet_lock::PacketLockComponent;
use hermes_relayer_components::relay::traits::packet_relayer::CanRelayPacket;
use hermes_relayer_components::with_default_relay_components;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetter, RuntimeTypeComponent};
use ibc_relayer::config::filter::PacketFilter as PacketFilterConfig;
use ibc_relayer_types::core::ics04_channel::packet::{Packet, Sequence};
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ClientId, PortId};

use crate::context::chain::WasmCosmosChain;

#[derive(Clone)]
pub struct WasmCosmosRelay {
    pub runtime: HermesRuntime,
    pub src_chain: WasmCosmosChain,
    pub dst_chain: WasmCosmosChain,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
    pub packet_filter: PacketFilterConfig,
    pub packet_lock_mutex: Arc<Mutex<BTreeSet<(ChannelId, PortId, ChannelId, PortId, Sequence)>>>,
}

impl WasmCosmosRelay {
    pub fn new(
        runtime: HermesRuntime,
        src_chain: WasmCosmosChain,
        dst_chain: WasmCosmosChain,
        src_client_id: ClientId,
        dst_client_id: ClientId,
        packet_filter: PacketFilterConfig,
    ) -> Self {
        Self {
            runtime,
            src_chain,
            dst_chain,
            src_client_id,
            dst_client_id,
            packet_filter,
            packet_lock_mutex: Arc::new(Mutex::new(BTreeSet::new())),
        }
    }
}

pub struct WasmCosmosRelayComponents;

delegate_components! {
    WasmCosmosRelayComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
            RetryableErrorComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeComponent:
            ProvideHermesRuntime,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideCosmosLogger,
        MaxErrorRetryGetterComponent:
            ReturnMaxRetry<3>,
        PacketLockComponent:
            ProvidePacketLockWithMutex,
    }
}

with_default_relay_components! {
    delegate_components! {
        WasmCosmosRelayComponents {
            @DefaultRelayComponents : DefaultRelayComponents,
        }
    }
}

impl HasComponents for WasmCosmosRelay {
    type Components = WasmCosmosRelayComponents;
}

pub trait CanUseWasmCosmosRelay:
    CanRelayPacket + CanBootstrapConnection + CanBootstrapChannel + CanRun
where
    Self::SrcChain:
        HasInitConnectionOptionsType<Self::DstChain> + HasInitChannelOptionsType<Self::DstChain>,
{
}

impl CanUseWasmCosmosRelay for WasmCosmosRelay {}

pub trait CanUseLogger: for<'a> CanLog<LogSkipRelayLockedPacket<'a, WasmCosmosRelay>> {}

impl CanUseLogger for CosmosLogger {}

impl ProvideRelayChains<WasmCosmosRelay> for WasmCosmosRelayComponents {
    type SrcChain = WasmCosmosChain;

    type DstChain = WasmCosmosChain;

    type Packet = Packet;

    fn src_chain(relay: &WasmCosmosRelay) -> &WasmCosmosChain {
        &relay.src_chain
    }

    fn dst_chain(relay: &WasmCosmosRelay) -> &WasmCosmosChain {
        &relay.dst_chain
    }

    fn src_client_id(relay: &WasmCosmosRelay) -> &ClientId {
        &relay.src_client_id
    }

    fn dst_client_id(relay: &WasmCosmosRelay) -> &ClientId {
        &relay.dst_client_id
    }
}

impl RuntimeGetter<WasmCosmosRelay> for WasmCosmosRelayComponents {
    fn runtime(relay: &WasmCosmosRelay) -> &HermesRuntime {
        &relay.runtime
    }
}

impl PacketFilter<WasmCosmosRelay> for WasmCosmosRelayComponents {
    async fn should_relay_packet(relay: &WasmCosmosRelay, packet: &Packet) -> Result<bool, Error> {
        Ok(relay
            .packet_filter
            .channel_policy
            .is_allowed(&packet.source_port, &packet.source_channel))
    }
}

impl PacketMutexGetter<WasmCosmosRelay> for WasmCosmosRelayComponents {
    fn packet_mutex(
        relay: &WasmCosmosRelay,
    ) -> &hermes_relayer_components::relay::impls::packet_lock::PacketMutex<WasmCosmosRelay> {
        &relay.packet_lock_mutex
    }
}
