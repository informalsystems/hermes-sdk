pub use cgp::prelude::*;
pub use cgp_async::async_trait;

pub trait Async: Send + Sync + 'static {}

impl<A> Async for A where A: Send + Sync + 'static {}
