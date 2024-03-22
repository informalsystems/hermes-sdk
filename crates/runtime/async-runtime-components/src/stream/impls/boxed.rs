use alloc::boxed::Box;
use core::pin::Pin;

use cgp_core::prelude::*;
use futures_core::stream::Stream;
use hermes_runtime_components::traits::stream::ProvideStreamType;

use crate::stream::traits::boxed::BoxedStreamTypeProvider;

pub struct ProvideBoxedStreamType;

impl<Runtime> ProvideStreamType<Runtime> for ProvideBoxedStreamType
where
    Runtime: Async,
{
    type Stream<Item: Async> = Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>;
}

impl<Runtime> BoxedStreamTypeProvider<Runtime> for ProvideBoxedStreamType
where
    Runtime: Async,
{
    fn to_boxed_stream<Item>(
        stream: Self::Stream<Item>,
    ) -> Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>
    where
        Item: Async,
    {
        stream
    }

    fn from_boxed_stream<Item>(
        stream: Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>,
    ) -> Self::Stream<Item>
    where
        Item: Async,
    {
        stream
    }
}
