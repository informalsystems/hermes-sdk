pub use cgp::prelude::*;

pub trait Async: Send + Sync + 'static {}

impl<A> Async for A where A: Send + Sync + 'static {}
