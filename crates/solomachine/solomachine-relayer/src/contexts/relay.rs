use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;
use hermes_relayer_components::components::default::relay::*;
use hermes_relayer_components::relay::traits::chains::ProvideRelayChains;
use hermes_relayer_components::relay::traits::connection::open_init::CanInitConnection;
use hermes_relayer_components::with_default_relay_components;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    GetRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::chain::MockSolomachine;

#[derive(HasField)]
pub struct SolomachineRelay {
    pub runtime: HermesRuntime,
    pub src_chain: MockSolomachine,
    pub dst_chain: CosmosChain,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
}

pub struct SolomachineRelayComponents;

with_default_relay_components! {
    delegate_components! {
        SolomachineRelayComponents {
            @DefaultRelayComponents : DefaultRelayComponents,
        }
    }
}

impl HasComponents for SolomachineRelay {
    type Components = SolomachineRelayComponents;
}

delegate_components! {
    SolomachineRelayComponents {
        RuntimeTypeComponent: ProvideHermesRuntime,
        RuntimeGetterComponent:
            GetRuntimeField<symbol!("runtime")>,
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DebugError,
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

impl ProvideRelayChains<SolomachineRelay> for SolomachineRelayComponents {
    type SrcChain = MockSolomachine;

    type DstChain = CosmosChain;

    type Packet = Packet;

    fn src_client_id(relay: &SolomachineRelay) -> &ClientId {
        &relay.src_client_id
    }

    fn dst_client_id(relay: &SolomachineRelay) -> &ClientId {
        &relay.dst_client_id
    }

    fn src_chain(relay: &SolomachineRelay) -> &MockSolomachine {
        &relay.src_chain
    }

    fn dst_chain(relay: &SolomachineRelay) -> &CosmosChain {
        &relay.dst_chain
    }
}

pub trait CanUseSolomachineRelay: CanInitConnection
where
    Self::SrcChain: HasInitConnectionOptionsType<Self::DstChain>,
{
}

impl CanUseSolomachineRelay for SolomachineRelay {}
