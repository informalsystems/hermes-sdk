use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::impls::use_field::UseField;
use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;
use hermes_relayer_components::components::default::relay::*;
use hermes_relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeAtComponent,
};
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::traits::chains::{
    DstClientIdGetterComponent, HasRelayClientIds, SrcClientIdGetterComponent,
};
use hermes_relayer_components::relay::traits::connection::open_init::CanInitConnection;
use hermes_relayer_components::with_default_relay_components;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
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
        [
            RuntimeTypeComponent,
            RuntimeGetterComponent,
        ]:
            ProvideDefaultRuntimeField,
        ErrorTypeComponent: ProvideHermesError,
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
        SrcClientIdGetterComponent:
            UseField<symbol!("src_client_id")>,
        DstClientIdGetterComponent:
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
