use core::pin::Pin;

use cgp_core::prelude::{Async, DelegateComponent};
use futures::prelude::Stream;
use ibc_relayer_components::runtime::traits::stream::{ProvideStreamType, StreamTypeComponent};

pub struct ProvideBoxedStreamType;

impl<Runtime> ProvideStreamType<Runtime> for ProvideBoxedStreamType
where
    Runtime: Async,
{
    type Stream<Item: Async> = Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>;
}

pub trait HasBoxedStreamType<Runtime>: ProvideStreamType<Runtime>
where
    Runtime: Async,
{
    fn to_boxed_stream<Item>(
        stream: Self::Stream<Item>,
    ) -> Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>
    where
        Item: Async;
}

impl<Runtime> HasBoxedStreamType<Runtime> for ProvideBoxedStreamType
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
}

impl<Runtime, Component, Delegate> HasBoxedStreamType<Runtime> for Component
where
    Runtime: Async,
    Component: DelegateComponent<StreamTypeComponent, Delegate = Delegate>,
    Delegate: HasBoxedStreamType<Runtime>,
{
    fn to_boxed_stream<Item>(
        stream: Self::Stream<Item>,
    ) -> Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>
    where
        Item: Async,
    {
        Delegate::to_boxed_stream(stream)
    }
}
