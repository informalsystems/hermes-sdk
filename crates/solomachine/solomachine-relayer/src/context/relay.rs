use cgp_core::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;
use hermes_relayer_components::relay::traits::connection::open_init::CanInitConnection;
use hermes_runtime::types::runtime::HermesRuntime;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::traits::solomachine::Solomachine;
use crate::types::chain::SolomachineChain;

#[derive(HasField)]
pub struct SolomachineRelay<Chain> {
    pub runtime: HermesRuntime,
    pub src_chain: SolomachineChain<Chain>,
    pub dst_chain: CosmosChain,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
}

impl<Chain> SolomachineRelay<Chain> {
    pub fn new(
        runtime: HermesRuntime,
        src_chain: SolomachineChain<Chain>,
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

impl<Chain> CanUseSolomachineRelay for SolomachineRelay<Chain> where
    Chain: Solomachine<Error = Error>
{
}
