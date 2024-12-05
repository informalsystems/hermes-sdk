use core::marker::PhantomData;

use cgp::prelude::*;

use crate::batch::traits::types::HasMessageBatchChannelTypes;

#[derive_component(MessageBatchSenderGetterComponent, MessageBatchSenderGetter<Relay>)]
pub trait HasMessageBatchSender<Tag>: HasMessageBatchChannelTypes<Tag> {
    fn get_batch_sender(&self, _tag: PhantomData<Tag>) -> &Self::MessageBatchSender;
}
