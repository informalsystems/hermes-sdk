use core::pin::Pin;

use cgp_core::traits::Async;
use futures::stream::Stream;
use ibc_relayer_components::runtime::traits::stream::HasStreamType;

use crate::types::runtime::TokioRuntimeContext;

impl HasStreamType for TokioRuntimeContext {
    type Stream<Item: Async> = Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>;
}
