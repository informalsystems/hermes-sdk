use cgp_core::prelude::*;
use hermes_cosmos_chain_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_relayer_components::chain::traits::types::chain_id::ProvideChainIdType;
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;
use hermes_relayer_components::chain::traits::types::height::{HasHeightType, ProvideHeightType};
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::ProvideMessageType;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::status::ProvideChainStatusType;
use hermes_relayer_components::chain::traits::types::timestamp::{
    HasTimestampType, TimestampTypeComponent,
};
use ibc_relayer_types::timestamp::Timestamp;

use crate::types::event::SovereignEvent;
use crate::types::height::RollupHeight;
use crate::types::message::SovereignMessage;
use crate::types::rollup_id::RollupId;
use crate::types::status::SovereignRollupStatus;

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

impl<Chain> ProvideChainStatusType<Chain> for ProvideSovereignChainTypes
where
    Chain: HasHeightType<Height = RollupHeight> + HasTimestampType<Timestamp = Timestamp>,
{
    type ChainStatus = SovereignRollupStatus;

    fn chain_status_height(status: &SovereignRollupStatus) -> &RollupHeight {
        &status.height
    }

    fn chain_status_timestamp(status: &Self::ChainStatus) -> &Timestamp {
        &status.timestamp
    }
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
