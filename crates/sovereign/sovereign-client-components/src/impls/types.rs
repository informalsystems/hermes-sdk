use cgp_core::prelude::*;
use hermes_cosmos_client_components::components::types::chain::ProvideCosmosChainTypes;
use hermes_relayer_components::chain::traits::types::chain_id::ProvideChainIdType;
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;
use hermes_relayer_components::chain::traits::types::height::ProvideHeightType;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::ProvideMessageType;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;

use crate::types::event::SovereignEvent;
use crate::types::height::RollupHeight;
use crate::types::message::SovereignMessage;
use crate::types::rollup_id::RollupId;

pub struct ProvideSovereignChainTypes;

impl<Chain> ProvideHeightType<Chain> for ProvideSovereignChainTypes
where
    Chain: Async,
{
    type Height = RollupHeight;
}

impl<Chain> ProvideChainIdType<Chain> for ProvideSovereignChainTypes
where
    Chain: Async,
{
    type ChainId = RollupId;
}

impl<Chain> ProvideMessageType<Chain> for ProvideSovereignChainTypes
where
    Chain: Async,
{
    type Message = SovereignMessage;
}

impl<Chain> ProvideEventType<Chain> for ProvideSovereignChainTypes
where
    Chain: Async,
{
    type Event = SovereignEvent;
}

delegate_components! {
    ProvideSovereignChainTypes {
        [
            TimestampTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
        ]:
            ProvideCosmosChainTypes,
    }
}
