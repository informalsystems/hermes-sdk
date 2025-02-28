use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::{UseField, WithField};
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_logger::UseHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeProviderComponent,
};
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;
use hermes_relayer_components::components::default::relay::*;
use hermes_relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeAtComponent,
};
use hermes_relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::traits::chains::HasRelayClientIds;
use hermes_relayer_components::relay::traits::connection::open_init::CanInitConnection;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use ibc::core::host::types::identifiers::ClientId;

use crate::contexts::chain::MockSolomachine;

#[cgp_context(SolomachineRelayComponents: DefaultRelayPreset)]
#[derive(HasField)]
pub struct SolomachineRelay {
    pub runtime: HermesRuntime,
    pub src_chain: MockSolomachine,
    pub dst_chain: CosmosChain,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
}

delegate_components! {
    SolomachineRelayComponents {
        RuntimeTypeProviderComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        [
            LoggerTypeProviderComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            UseHermesLogger,
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
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
    }
}

impl SolomachineRelay {
    pub fn new(
        runtime: HermesRuntime,
        src_chain: MockSolomachine,
        dst_chain: CosmosChain,
        src_client_id: ClientId,
        dst_client_id: ClientId,
    ) -> Self {
        Self {
            runtime,
            src_chain,
            dst_chain,
            src_client_id,
            dst_client_id,
        }
    }
}

pub trait CanUseSolomachineRelay: HasRelayClientIds + CanInitConnection
where
    Self::SrcChain: HasInitConnectionOptionsType<Self::DstChain>,
{
}

impl CanUseSolomachineRelay for SolomachineRelay {}
