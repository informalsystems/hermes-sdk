use alloc::sync::Arc;
use cgp_core::Async;
use ibc_relayer::chain::endpoint::ChainStatus;
use ibc_relayer_components::chain::traits::types::height::{HasHeightType, HeightTypeProvider};
use ibc_relayer_components::chain::traits::types::message::MessageTypeProvider;
use ibc_relayer_components::chain::traits::types::status::ChainStatusTypeProvider;
use ibc_relayer_components::chain::traits::types::timestamp::HasTimestampType;
use ibc_relayer_types::timestamp::Timestamp;
use ibc_relayer_types::Height;

use crate::traits::message::CosmosMessage;

pub struct ProvideCosmosChainTypes;

impl<Chain> HeightTypeProvider<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Height = Height;
}

impl<Chain> MessageTypeProvider<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Message = Arc<dyn CosmosMessage>;
}

impl<Chain> ChainStatusTypeProvider<Chain> for ProvideCosmosChainTypes
where
    Chain: HasHeightType<Height = Height> + HasTimestampType<Timestamp = Timestamp>,
{
    type ChainStatus = ChainStatus;

    fn chain_status_height(status: &ChainStatus) -> &Height {
        &status.height
    }

    fn chain_status_timestamp(status: &ChainStatus) -> &Timestamp {
        &status.timestamp
    }
}
