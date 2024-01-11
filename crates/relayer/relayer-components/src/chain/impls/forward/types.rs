use cgp_core::HasInner;

use crate::chain::traits::types::chain::HasChainTypes;
use crate::chain::traits::types::chain_id::{HasChainIdType, ProvideChainIdType};
use crate::chain::traits::types::event::{HasEventType, ProvideEventType};
use crate::chain::traits::types::height::{HasHeightType, ProvideHeightType};
use crate::chain::traits::types::ibc::{HasIbcChainTypes, ProvideIbcChainTypes};
use crate::chain::traits::types::message::{HasMessageType, ProvideMessageType};
use crate::chain::traits::types::timestamp::{HasTimestampType, ProvideTimestampType};

pub struct ForwardChainTypes;

impl<Chain, Inner> ProvideHeightType<Chain> for ForwardChainTypes
where
    Chain: HasInner<Inner = Inner>,
    Inner: HasHeightType,
{
    type Height = Inner::Height;
}

impl<Chain, Inner> ProvideMessageType<Chain> for ForwardChainTypes
where
    Chain: HasInner<Inner = Inner>,
    Inner: HasMessageType,
{
    type Message = Inner::Message;
}

impl<Chain, Inner> ProvideEventType<Chain> for ForwardChainTypes
where
    Chain: HasInner<Inner = Inner>,
    Inner: HasEventType,
{
    type Event = Inner::Event;
}

impl<Chain, Inner> ProvideChainIdType<Chain> for ForwardChainTypes
where
    Chain: HasInner<Inner = Inner>,
    Inner: HasChainIdType,
{
    type ChainId = Inner::ChainId;
}

impl<Chain, Inner> ProvideTimestampType<Chain> for ForwardChainTypes
where
    Chain: HasInner<Inner = Inner>,
    Inner: HasTimestampType,
{
    type Timestamp = Inner::Timestamp;
}

impl<Chain, Counterparty, Inner> ProvideIbcChainTypes<Chain, Counterparty> for ForwardChainTypes
where
    Chain: HasInner<Inner = Inner> + HasChainTypes,
    Inner: HasIbcChainTypes<Counterparty>,
{
    type ClientId = Inner::ClientId;

    type ConnectionId = Inner::ConnectionId;

    type ChannelId = Inner::ChannelId;

    type PortId = Inner::PortId;

    type Sequence = Inner::Sequence;
}
