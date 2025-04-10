# Check Components

A key highlight of CGP is that all wiring of components are done lazily only when a consumer trait is used with a concrete context. This flexibility makes it easy to write modular code, but can be challenging when there are errors in the wiring. The `#[cgp_provider]` `check_components!` macro can be used to check for the correctness of wiring for a certain component.

## `#[cgp_provider]` and `IsProviderFor`

The `#[cgp_provider]` macro is used to implement the `IsProviderFor` trait for a provider implementation. The `IsProvider` trait has a dummy trait interface that can be trivially implemented:

```rust
pub trait IsProviderFor<Component, Context, Params = ()> {}
```

The trick for CGP is that instead of implementing the `IsProviderFor` trait trivially, we implement it with the same constraint that we would have needed to implement a given provider trait. By doing so, the constraints for the provider trait implementation is "carried" by `IsProviderFor`, so that Rust can use it to display more informative errors when there are incorrect wirings.

The `Params` generic parameter is used when a provider trait contains additional generic parameters. When there is no generic parameters, the unit type `()` is applied by default.

As an example, suppose that we are implementing a chain method for querying the chain height, the trait would be defined as:

```rust
#[cgp_component {
    provider: ChainHeightQuerier,
}]
pub trait CanQueryChainHeight: HasHeightType + HasErrorType {
    fn query_chain_height(&self) -> Result<Self::Height, Self::Error>;
}
```

A context-generic provider for the chain height querier would require additional dependencies, such as access to an RPC client.

```rust
pub struct QueryChainHeightWithRpcClient;

#[cgp_provider(ChainHeightQuerierComponent)]
impl<Chain> ChainHeightQuerier<Chain> for QueryChainHeightWithRpcClient
where
    Chain: HasHeightType + HasRpcClient + CanRaiseError<RpcError>,
{
    fn query_chain_height(chain: &Chain) -> Result<Chain::Height, Chain::Error> {
        ...
    }
}
```

Behind the scene, `#[cgp_provider]` would generate the following `IsProviderFor` implementation:

```rust
impl<Chain> IsProviderFor<ChainHeightQuerierComponent, Chain>
    for QueryChainHeightWithRpcClient
where
    Chain: HasHeightType + HasRpcClient + CanRaiseError<RpcError>,
{ }
```

That is, we implement `IsProviderFor<ChainHeightQuerierComponent, Chain>` for the provider `QueryChainHeightWithRpcClient` with the same constraints it use to implement the provider trait of `ChainHeightQuerierComponent`, which is `ChainHeightQuerier`.

## `CanUseComponent` Trait

The `IsProviderFor` trait is implement by a provider type. But when checking for the component wiring on a context, we want to check whether the specific provider that is wired with a context implements `IsProviderFor`. To facilitate such check, we define a `CanUseComponent` trait alias to help us perform the check:

```rust
pub trait CanUseComponent<Component, Params = ()> {}

impl<Context, Component, Params> CanUseComponent<Component, Params> for Context
where
    Context: HasProvider,
    Context::Provider: IsProviderFor<Component, Context, Params>,
{ }
```

To put it simply, `CanUseComponent<Component, Params>` is automatically implemented for a `Context` type, if `Context::Provider` implements `IsProviderFor<Component, Context, Params>`.

## Check Components

To check whether a context implements a component, we now just need to check whether the context implements `CanUseComponent`. One way we can do that is by defining a check trait to check for that manually. Supposed we have the following context:

```rust
#[cgp_context(MyChainComponents)]
pub struct MyChain {
    pub rpc_client: RpcClient,
    ...
}

delegate_components! {
    MyChainComponents {
        ErrorRaiserComponent:
            HandleChainErrors,
        HeightTypeProviderComponent:
            UseType<u64>,
        RpcClientGetterComponent:
            UseField<symbol!("rpc_client")>,
        ChainHeightQuerierComponent:
            QueryChainHeightWithRpcClient,
    }
}
```

```rust
pub trait CanUseMyChainComponents<Component, Params = ()>: CanUseComponent<Component, Params> {}

impl CanUseMyChainComponents<ChainHeightQuerierComponent> {}

impl CanUseMyChainComponents<HeightTypeProviderComponent> {}

impl CanUseMyChainComponents<RpcClientGetterComponent> {}

impl CanUseMyChainComponents<ErrorRaiserComponent, RpcError> {}
```

We define a trait `CanUseMyChainComponents<Component, Params>`, which simply requires `CanUseComponent<Component, Params>` as its supertrait. By doing so, we can only implement `CanUseMyChainComponents<Component, Params>` if `CanUseComponent<Component, Params>` is also implemented. But given that `CanUseComponent` is automatically implemented if the wiring is correct, any incorrect wiring would result in an error that will be caught when we try to implement `CanUseMyChainComponents`. Thus, each `impl` of `CanUseMyChainComponents` is an _assertion_ that `CanUseComponent` is implemented for the given component and params.

The above example performs 3 separate assertions:
- `CanUseMyChainComponents<ChainHeightQuerierComponent>` asserts that `QueryChainHeightWithRpcClient` implements `ChainHeightQuerier<MyChain>`.
- `CanUseMyChainComponents<RpcClientGetterComponent>` asserts that `UseField<symbol!("rpc_client")>` implements `RpcClientGetter<MyChain>`.
- `CanUseMyChainComponents<ErrorRaiserComponent, RpcError>` asserts that `HandleChainErrors` implements `ErrorRaiser<MyChain, RpcError>`.

By having multiple separate assertion impls, when there is any unsatisfied constraint, only the affected assertions would be highlighted with errors. For example, if `HandleChainErrors` does not implement `ErrorRaiser<MyChain, RpcError>`, then the impl for `CanUseMyChainComponents<ErrorRaiserComponent, RpcError>` would produce error, but also `CanUseMyChainComponents<ChainHeightQuerierComponent>`, since `QueryChainHeightWithRpcClient` requires `MyChain` to implement `CanRaiseError<RpcError>`. On the other hand, there would be no error highlighted for `CanUseMyChainComponents<RpcClientGetterComponent>`. As a result, we can more easily identify the source of errors, if we have fine grained checking of as many components in the wiring.

## `check_components!` Macro

The `check_components!` macro is provided to simplify the definition and implementation of check traits such as `CanUseMyChainComponents` above. For example, the same checks can be performed as follows:

```rust
check_components! {
    CanUseMyChainComponents for MyChain {
        ChainHeightQuerierComponent,
        HeightTypeProviderComponent,
        RpcClientGetterComponent,
        ErrorRaiserComponent: RpcError,
    }
}
```

Behind the scene, `check_components!` generates the same trait implementations as we have implemented earlier. But now the definition is more concise and easy to understand. When there is any unsatisfied constraint, only the affected lines will be highlighted, thus making it easier to diagnose what went wrong.

When there are additional generic parameters in the provider trait, such as for `ErrorRaiserComponent`, they would be specified after `:` on a component entry. The pattern allows for multiplexing the implementation so that we can more easily check against the implementation of multiple generic parameters. For example:

```rust
check_components! {
    CanUseMyChainComponents for MyChain {
        ChainHeightQuerierComponent,
        HeightTypeProviderComponent,
        RpcClientGetterComponent,
        ErrorRaiserComponent: [
            String,
            RpcError,
        ],
    }
}
```

is the same as writing:

```rust
check_components! {
    CanUseMyChainComponents for MyChain {
        ChainHeightQuerierComponent,
        HeightTypeProviderComponent,
        RpcClientGetterComponent,
        ErrorRaiserComponent: String,
        ErrorRaiserComponent: RpcError,
    }
}
```

Similarly, the check multiplexing can be done on the component entry, so that multiple components are checked against the same generic parameters. This would be useful for example to check for counterparty-specific chain implementations:

```rust
check_components! {
    CanUseMyChainComponents for MyChain {
        ChainHeightQuerierComponent,
        HeightTypeProviderComponent,
        RpcClientGetterComponent,
        [
            ClientStateQuerierComponent,
            ConsensusStateQuerierComponent,
        ]:
            MyOtherChain,
        ErrorRaiserComponent: [
            String,
            RpcError,
        ],
    }
}
```

The above check is the same as writing:

```rust
check_components! {
    CanUseMyChainComponents for MyChain {
        ChainHeightQuerierComponent,
        HeightTypeProviderComponent,
        RpcClientGetterComponent,
        ClientStateQuerierComponent: MyOtherChain,
        ConsensusStateQuerierComponent: MyOtherChain,
        ErrorRaiserComponent: String,
        ErrorRaiserComponent: RpcError,
    }
}
```

We can even perform multiplexing checks on both the component entry and generic parameters. For example:

```rust
check_components! {
    CanUseMyEncoding for MyEncoding {
        [
            EncoderComponent,
            DecoderComponent,
        ]: [
            (ViaProtobuf, MyClientState),
            (ViaProtobuf, MyConsensusState),
        ],
    }
}
```

is equivalent to writing:

```rust
check_components! {
    CanUseMyEncoding for MyEncoding {
        EncoderComponent: (ViaProtobuf, MyClientState),
        EncoderComponent: (ViaProtobuf, MyConsensusState),
        DecoderComponent: (ViaProtobuf, MyClientState),
        DecoderComponent: (ViaProtobuf, MyConsensusState),
    }
}
```

## Legacy Checks

Before `IsProvider` and `check_components!` are available, we used a simpler check trait pattern to check for the implementation of consumer traits. For example, we would write the example check trait earlier as:

```rust
pub trait CanUseMyChain:
    CanQueryChainHeight
    + HasRpcClient
    + CanRaiseError<RpcError>
{}

impl CanUseMyChain for MyChain {}
```

This legacy approach for checking also works, but has a few weaknesses. Most importantly, when there is any unsatisfied coinstraint, the error will be highlighted on the line `impl CanUseMyChain for MyChain`, making it more challenging to track which consumer trait does the error correspond to.

Furthermore, if a consumer trait contains additional `where` clause, then that would also need to be manually propagated on the check trait. On the other hand, the use of `CanUseComponent` hides away any potential `where` clause that may be applicable, making it more easy for us to define the checks without needing to know the details of both the consumer trait and the provider trait.

Because of this, we should migrate to using `check_components!` whenever possible to check for the component wiring.

## Abstract Type Checks

There is still one place where the use of legacy check pattern would still be useful, which is to check for the instantiation of the abstract types for our concrete context. For example, we can check that `MyChain` implements `HasHeightType` using `u64` as the height with the following check:

```rust
pub trait CanUseMyChain:
    HasHeightType<Height = u64>
{}

impl CanUseMyChain for MyChain {}
```

The check above makes it informative for the readers to more easily figure out the concrete type instantiation of each abstract type. It can also be useful if we want to double check if we are instantiating an abstract type correctly.

On the other hand, we cannot perform checks on the associated type when using `check_components!`, and can only know that it contains an instantiated `Height` type, but not which concrete type it is.

```rust
check_components! {
    CanUseMyChainComponents for MyChain {
        HeightTypeProviderComponent,
        ...
    }
}
```

As a result, we would keep the legacy check pattern to be used for checking the instantiation of abstract types.
