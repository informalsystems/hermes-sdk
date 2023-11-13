use cgp_core::prelude::*;

pub trait TestChainContext: Async {
    type Chain: Async;
}

pub trait TestChainTarget<Test> {
    type TestChain: TestChainContext;
}
