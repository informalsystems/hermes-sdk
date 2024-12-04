use cgp::prelude::*;
use hermes_chain_type_components::traits::types::message::HasMessageType;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_relayer_components::chain::traits::types::chain::HasChainTypes;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::relay::traits::target::{HasTargetChainTypes, RelayTarget};
use hermes_runtime_components::traits::channel::HasChannelTypes;
use hermes_runtime_components::traits::channel_once::HasChannelOnceTypes;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::batch::types::aliases::MessageBatchSender;

#[derive_component(MessageBatchSenderGetterComponent, MessageBatchSenderGetter<Relay>)]
pub trait HasMessageBatchSender<Target: RelayTarget>:
    HasTargetChainTypes<
        Target,
        TargetChain: HasMessageType
                         + HasMessageResponseType
                         + HasRuntime<Runtime: HasChannelTypes + HasChannelOnceTypes>,
    > + HasErrorType
{
    fn get_batch_sender(&self) -> &MessageBatchSender<Self::TargetChain, Self::Error>;
}

pub trait HasMessageBatchSenderType<Error>:
    HasChainTypes + HasRuntime<Runtime: HasChannelTypes + HasChannelOnceTypes>
{
}

impl<Chain, Error> HasMessageBatchSenderType<Error> for Chain
where
    Chain: HasChainTypes + HasRuntime,
    Chain::Runtime: HasChannelTypes + HasChannelOnceTypes,
{
}

pub trait HasMessageBatchSenderTypes:
    HasRelayChains<
    SrcChain: HasMessageBatchSenderType<Self::Error>,
    DstChain: HasMessageBatchSenderType<Self::Error>,
>
{
}

impl<Relay> HasMessageBatchSenderTypes for Relay
where
    Relay: HasRelayChains,
    Relay::SrcChain: HasMessageBatchSenderType<Relay::Error>,
    Relay::DstChain: HasMessageBatchSenderType<Relay::Error>,
{
}
