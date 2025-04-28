use core::marker::PhantomData;

use cgp::core::field::UseField;
use hermes_prelude::*;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::impls::SelectRelayChains;

use crate::batch::traits::types::HasMessageBatchChannelTypes;

#[cgp_component {
  name: MessageBatchSenderGetterComponent<Tag>,
  provider: MessageBatchSenderGetter,
}]
pub trait HasMessageBatchSender<Tag>: HasMessageBatchChannelTypes<Tag> {
    fn get_batch_sender(&self, _tag: PhantomData<Tag>) -> &Self::MessageBatchSender;
}

#[cgp_provider(MessageBatchSenderGetterComponent<SenderTag>)]
impl<Context, SenderTag, FieldTag> MessageBatchSenderGetter<Context, SenderTag>
    for UseField<FieldTag>
where
    Context: HasMessageBatchChannelTypes<SenderTag>
        + HasField<FieldTag, Value = Context::MessageBatchSender>,
{
    fn get_batch_sender(
        context: &Context,
        _tag: PhantomData<SenderTag>,
    ) -> &Context::MessageBatchSender {
        context.get_field(PhantomData)
    }
}

#[cgp_provider(MessageBatchSenderGetterComponent<Src>)]
impl<Relay, SrcTag, DstTag, Sender> MessageBatchSenderGetter<Relay, Src>
    for SelectRelayChains<SrcTag, DstTag>
where
    Relay: HasMessageBatchSender<SrcTag, MessageBatchSender = Sender>
        + HasMessageBatchChannelTypes<Src, MessageBatchSender = Sender>,
{
    fn get_batch_sender(context: &Relay, _tag: PhantomData<Src>) -> &Sender {
        context.get_batch_sender(PhantomData::<SrcTag>)
    }
}

#[cgp_provider(MessageBatchSenderGetterComponent<Dst>)]
impl<Relay, SrcTag, DstTag, Sender> MessageBatchSenderGetter<Relay, Dst>
    for SelectRelayChains<SrcTag, DstTag>
where
    Relay: HasMessageBatchSender<DstTag, MessageBatchSender = Sender>
        + HasMessageBatchChannelTypes<Dst, MessageBatchSender = Sender>,
{
    fn get_batch_sender(context: &Relay, _tag: PhantomData<Dst>) -> &Sender {
        context.get_batch_sender(PhantomData::<DstTag>)
    }
}
