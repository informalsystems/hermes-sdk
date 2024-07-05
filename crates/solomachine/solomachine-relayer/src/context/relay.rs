use cgp_core::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;
use hermes_relayer_components::relay::traits::connection::open_init::CanInitConnection;
use hermes_runtime::types::runtime::HermesRuntime;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::context::chain::MockSolomachine;
use crate::traits::solomachine::Solomachine;

#[derive(HasField)]
pub struct SolomachineRelay {
    pub runtime: HermesRuntime,
    pub src_chain: MockSolomachine,
    pub dst_chain: CosmosChain,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
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

pub trait CanUseSolomachineRelay: CanInitConnection
where
    Self::SrcChain: HasInitConnectionOptionsType<Self::DstChain>,
{
}

impl CanUseSolomachineRelay for SolomachineRelay {}
