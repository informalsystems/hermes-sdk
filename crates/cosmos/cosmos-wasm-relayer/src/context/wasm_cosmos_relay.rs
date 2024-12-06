use std::collections::BTreeSet;
use std::sync::Arc;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::impls::use_field::{UseField, WithField};
use cgp::core::types::impls::WithType;
use cgp::extra::run::CanRun;
use cgp::prelude::*;
use futures::lock::Mutex;
use hermes_cosmos_chain_components::impls::relay::packet_filter::FilterPacketWithConfig;
use hermes_cosmos_chain_components::types::messages::packet::packet_filter::PacketFilterConfig;
use hermes_cosmos_relayer::impls::error::HandleCosmosError;
use hermes_logger::{HermesLogger, ProvideHermesLogger};
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
use hermes_relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeAtComponent,
};
use hermes_relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::impls::packet_lock::{
    PacketMutexGetterComponent, ProvidePacketLockWithMutex,
};
use hermes_relayer_components::relay::impls::packet_relayers::general::lock::LogSkipRelayLockedPacket;
use hermes_relayer_components::relay::traits::packet_filter::PacketFilterComponent;
use hermes_relayer_components::relay::traits::packet_lock::PacketLockComponent;
use hermes_relayer_components::relay::traits::packet_relayer::CanRelayPacket;
use hermes_relayer_components::with_default_relay_components;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetterComponent, RuntimeTypeComponent};
use ibc::core::host::types::identifiers::{ChannelId, ClientId, PortId, Sequence};

use crate::context::chain::WasmCosmosChain;

#[derive(HasField, Clone)]
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
        RuntimeTypeComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideHermesLogger,
        [
            ChainTypeAtComponent<Src>,
            ChainGetterAtComponent<Src>,
        ]:
            UseField<symbol!("src_chain")>,
        [
            ChainTypeAtComponent<Dst>,
            ChainGetterAtComponent<Dst>,
        ]:
            UseField<symbol!("dst_chain")>,
        ClientIdAtGetterComponent<Src, Dst>:
            UseField<symbol!("src_client_id")>,
        ClientIdAtGetterComponent<Dst, Src>:
            UseField<symbol!("dst_client_id")>,
        PacketMutexGetterComponent:
            UseField<symbol!("packet_lock_mutex")>,
        PacketFilterComponent:
            FilterPacketWithConfig<symbol!("packet_filter")>,
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

impl CanUseLogger for HermesLogger {}
