use std::collections::BTreeSet;
use std::sync::Arc;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::UseField;
use cgp::extra::run::CanRun;
use futures::lock::Mutex;
use hermes_core::logging_components::traits::{CanLog, LoggerComponent};
use hermes_core::relayer_components::chain::traits::{
    HasInitChannelOptionsType, HasInitConnectionOptionsType,
};
use hermes_core::relayer_components::components::default::*;
use hermes_core::relayer_components::error::traits::RetryableErrorComponent;
use hermes_core::relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeProviderAtComponent,
};
use hermes_core::relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_core::relayer_components::multi::types::tags::{Dst, Src};
use hermes_core::relayer_components::relay::impls::{
    CanBootstrapChannel, CanBootstrapConnection, LogSkipRelayLockedPacket,
    PacketMutexGetterComponent,
};
use hermes_core::relayer_components::relay::traits::CanRelayPacket;
use hermes_core::runtime_components::traits::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_cosmos_chain_components::types::PacketFilterConfig;
use hermes_cosmos_relayer::impls::HandleCosmosError;
use hermes_prelude::*;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_tracing_logging_components::contexts::TracingLogger;
use ibc::core::host::types::identifiers::{ChannelId, ClientId, PortId, Sequence};

use crate::context::chain::WasmCosmosChain;

#[cgp_context(WasmCosmosRelayComponents: DefaultRelayPreset)]
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

delegate_components! {
    WasmCosmosRelayComponents {
        [
            ErrorTypeProviderComponent,
            ErrorRaiserComponent,
            RetryableErrorComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeProviderComponent: UseType<HermesRuntime>,
        RuntimeGetterComponent: UseField<symbol!("runtime")>,
        LoggerComponent: TracingLogger,
        [
            ChainTypeProviderAtComponent<Src>,
            ChainGetterAtComponent<Src>,
        ]:
            UseField<symbol!("src_chain")>,
        [
            ChainTypeProviderAtComponent<Dst>,
            ChainGetterAtComponent<Dst>,
        ]:
            UseField<symbol!("dst_chain")>,
        ClientIdAtGetterComponent<Src, Dst>:
            UseField<symbol!("src_client_id")>,
        ClientIdAtGetterComponent<Dst, Src>:
            UseField<symbol!("dst_client_id")>,
        PacketMutexGetterComponent:
            UseField<symbol!("packet_lock_mutex")>,
    }
}

pub trait CanUseWasmCosmosRelay:
    CanRelayPacket
    + CanBootstrapConnection
    + CanBootstrapChannel
    + CanRun
    + for<'a> CanLog<LogSkipRelayLockedPacket<'a, WasmCosmosRelay>>
where
    Self::SrcChain:
        HasInitConnectionOptionsType<Self::DstChain> + HasInitChannelOptionsType<Self::DstChain>,
{
}

impl CanUseWasmCosmosRelay for WasmCosmosRelay {}
