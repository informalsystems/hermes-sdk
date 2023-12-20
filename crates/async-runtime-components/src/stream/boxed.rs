use core::pin::Pin;

use cgp_core::prelude::*;
use futures_core::stream::Stream;
use ibc_relayer_components::runtime::traits::stream::{
    HasStreamType, ProvideStreamType, StreamTypeComponent,
};

pub struct ProvideBoxedStreamType;

impl<Runtime> ProvideStreamType<Runtime> for ProvideBoxedStreamType
where
    Runtime: Async,
{
    type Stream<Item: Async> = Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>;
}

pub trait HasBoxedStreamType: HasStreamType {
    fn to_boxed_stream<Item>(
        stream: Self::Stream<Item>,
    ) -> Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>
    where
        Item: Async;

    fn from_boxed_stream<Item>(
        stream: Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>,
    ) -> Self::Stream<Item>
    where
        Item: Async;
}

impl<Runtime, Components, Delegate> HasBoxedStreamType for Runtime
where
    Runtime: HasComponents<Components = Components>,
    Components: DelegateComponent<StreamTypeComponent, Delegate = Delegate>,
    Delegate: BoxedStreamTypeProvider<Runtime>,
{
    fn to_boxed_stream<Item>(
        stream: Self::Stream<Item>,
    ) -> Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>
    where
        Item: Async,
    {
        Components::to_boxed_stream(stream)
    }

    fn from_boxed_stream<Item>(
        stream: Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>,
    ) -> Self::Stream<Item>
    where
        Item: Async,
    {
        Components::from_boxed_stream(stream)
    }
}

pub trait BoxedStreamTypeProvider<Runtime>: ProvideStreamType<Runtime>
where
    Runtime: Async,
{
    fn to_boxed_stream<Item>(
        stream: Self::Stream<Item>,
    ) -> Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>
    where
        Item: Async;

    fn from_boxed_stream<Item>(
        stream: Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>,
    ) -> Self::Stream<Item>
    where
        Item: Async;
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

impl<Runtime, Component, Delegate> BoxedStreamTypeProvider<Runtime> for Component
where
    Runtime: Async,
    Component: DelegateComponent<StreamTypeComponent, Delegate = Delegate>,
    Delegate: BoxedStreamTypeProvider<Runtime>,
{
    fn to_boxed_stream<Item>(
        stream: Self::Stream<Item>,
    ) -> Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>
    where
        Item: Async,
    {
        Delegate::to_boxed_stream(stream)
    }

    fn from_boxed_stream<Item>(
        stream: Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>,
    ) -> Self::Stream<Item>
    where
        Item: Async,
    {
        Delegate::from_boxed_stream(stream)
    }
}
