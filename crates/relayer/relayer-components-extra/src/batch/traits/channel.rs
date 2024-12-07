use core::marker::PhantomData;

use cgp::core::field::impls::use_field::UseField;
use cgp::prelude::*;

use crate::batch::traits::types::HasMessageBatchChannelTypes;

#[derive_component(MessageBatchSenderGetterComponent<Tag>, MessageBatchSenderGetter<Context>)]
pub trait HasMessageBatchSender<Tag>: HasMessageBatchChannelTypes<Tag> {
    fn get_batch_sender(&self, _tag: PhantomData<Tag>) -> &Self::MessageBatchSender;
}

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
