# `UseDelegate` Pattern

`UseDelegate` is an advanced CGP pattern that can be used to perform "static dispatch" of methods and types based on one or more given generic parameters.

## Overview

This section provides an overview of how the `UseDelegate` pattern works. A CGP trait may contain one or more generic parameters such as follows:

```rust
#[cgp_type {
    provider: ChainTypeProviderAt,
}]
pub trait HasChainTypeAt<I> {
    type Chain;
}

#[cgp_getter {
    provider: ChainGetterAt,
}]
pub trait HasChainAt<I>: HasChainTypeAt<I> {
    fn chain(&self) -> &Self::Chain;
}
```

With the generic parameters, this means that a trait like `HasChainTypeAt<I>` may have _different_ `Chain` associated type based on the generic parameter `I`. Similarly, the trait `HasChainAt` may have different `chain` accessor method based on the generic parameter `I`. Note that in general, the method can not just be a getter, but also general methods such as for querying.

When we implement context-generic providers, we may not care about the generic parameter `I`, and provide it for _any_ `I` parameter that the context chooses. For example:

```rust
#[cgp_new_provider(ChainTypeProviderAtComponent)]
impl<Chain, I> ChainTypeProviderAt<Chain, I> for UseFooChain {
    type Chain = FooChain;
}

#[cgp_new_provider(ChainTypeProviderAtComponent)]
impl<Chain, I> ChainTypeProviderAt<Chain, I> for UseBarChain {
    type Chain = BarChain;
}
```

Note that the same implementation can be done with just `UseType<FooChain>` and `UseType<BarChain>`, but we are just showing the manual provider implementations to demonstrate the general case.

With multiple providers, we may want to wire up different provider for different index `I`. One way we can do that is by implementing the trait on a `UseDelegate` type.

```rust
pub struct UseDelegate<Components>(pub PhantomData<Components>);


#[cgp_provider(ChainTypeProviderAtComponent)]
impl<Chain, I, Delegate> ChainTypeProvider<Chain, I> for UseDelegate<Components>
where
    Components: DelegateComponent<I, Delegate = Delegate>,
    Delegate: ChainTypeProvider<Chain, I>,
{
    type Chain = Delegate::Chain;
}
```

We can understand the `UseDelegate` implementation above as a form of "table lookup" using the `Components` type. That is, `UseDelegate<Components>` implements `ChainTypeProvider<Chain, I>`, if `Components` "contains" an entry for `I`, and that entry _also_ implements `ChainTypeProvider<I>`.

Using `UseDelegate`, we can for example define custom mappings for our context like:

```rust
pub struct MyChainMapping;

delegate_components! {
    MyChainMapping {
        Index<0>: UseFooChain,
        Index<1>: UseBarChain,
    }
}

#[cgp_context(MyAppComponents)]
pub struct MyApp;

delegate_components! {
    MyAppComponents {
        ChainTypeProviderAtComponent: UseDelegate<MyChainMapping>,
        ...
    }
}
```

In the above example, we created a new "table" called `MyChainMapping`, which maps the type parameter `I` to a specific provider. So when `Index<0>` is used, we want to use `FooChain` as the `Chain` type, and when `Index<1>` is used, we want to use `BarChain` as the chain type.

Now even though `MyChainMapping` contains the mapping of `I`, it does _not_ implement `ChainTypeProviderAt`. After all, the blanket implementation of `ChainTypeProviderAt` is not implemented for any type that implement `DelegateComponent` that contains arbitrary mappings. However, by wrapping the mapping to become `UseDelegate<MyChainMapping>`, the composite type now implements `ChainTypeProviderAt`, since we have such a blanket implementation for `UseDelegate`.

With the given mapping, we can now verify that the context `MyApp` implements `HasChainTypeAt<Index<0>, Chain = FooChain>`, and `HasChainTypeAt<Index<1>, Chain = BarChain>`.

## Alternative Approach

An alternative approach to using `UseDelegate` is that we can include the generic parameters also in the component name type, such as:

```rust
#[cgp_type {
    provider: ChainTypeProviderAt<I>,
}]
pub trait HasChainTypeAt<I> {
    type Chain;
}
```

This way, the blanket implementation of `ChainTypeProviderAt` would also follows the specific generic parameter of the component name entry in `DelegateComponent`. So our mapping can work as follows:

```rust
pub struct MyChainMapping;

delegate_components! {
    MyChainMapping {
        ChainTypeProviderAtComponent<Index<0>>: UseFooChain,
        ChainTypeProviderAtComponent<Index<1>>: UseBarChain,
    }
}
```

Now instead of delegating based on `I`, we delegate based on `ChainTypeProviderAt<Index<0>>` inside of `MyChainMapping`. With this, `MyChainMapping` itself also implements `ChainTypeProviderAt<Index<0>>` and `ChainTypeProviderAt<Index<1>>`.

We can then still bulk delegate _all_ wirings of `ChainTypeProviderAtComponent<I>` inside `MyAppComponents` as follows:

```rust
#[cgp_context(MyAppComponents)]
pub struct MyApp;

delegate_components! {
    MyAppComponents {
        <I> ChainTypeProviderAtComponent<I>: MyChainMapping,
        ...
    }
}
```

With the `<I>`, parameter at the front, we are introducing a generic `I` inside the implementation of `DelegateComponent<ChainTypeProviderAtComponent<I>>`. So if there are a lot of "entries" inside of `MyChainMapping`, then this can save a lot of code repetition.

When the number of implementations are small, such as 2 in the example, it is also common to have the mapping done in the main context components, such as:

```rust
#[cgp_context(MyAppComponents)]
pub struct MyApp;

delegate_components! {
    MyAppComponents {
        ChainTypeProviderAtComponent<Index<0>>: UseFooChain,
        ChainTypeProviderAtComponent<Index<1>>: UseBarChain,
        ...
    }
}
```

Which approach is used depends whether we want to reuse the same mappings for multiple concrete contexts.

## Comparison with Both Approaches

In Hermes SDK, we use both patterns to dispatch the providers based on generic parameters. Usually, when the number of parameter instances are small, then we dispatch straight on the component name type. But when there are many possible parameter instances used, we dispatch through `UseDelegate`.

From a usability stand point, `UseDelegate` tends to lead to cleaner mapping, as we don't need to repeat the same component name for every mapping. However, since `UseDelegate` requires defining separate mapping structs, it may become tedious if we only want to dispatch 2 or 3 parameter instances.

Practically, there is also an important advantage of using `UseDelegate`, which is that it has looser ownership restriction and thus allows for more extensible mapping to be provided. Recall that `UseDelegate` uses `DelegateComponent` to look up which provider to dispatch to. Usually, `DelegateComponent` can only be implemented when the mapping struct is owned by the crate. But we can also implement `DelegateComponent<Components>` on a _foreign_ mapping struct, if we own the `Components` type in the generic parameter.

For example, even if we are in a foreign crate, we can extend `MyMapping` such as follows:

```rust
// In a different crate

pub struct AtBar;

delegate_components! {
    MyChainMapping {
        AtBaz: UseBazType,
    }
}
```

In this example, since we are defining a new struct `AtBar` that is owned by the crate, we can still "extend" the mapping of `MyChainMapping` which is owned by another crate.

This extension pattern is used at the core of `CosmosChain`, so that the counterparty-related components can be extended based on which chain is used for the `Counterparty` generic parameter.

## Future Improvements

Currently, `UseDelegate` needs to be implemented manually for every trait which we want to use it on. This creates a bit of noise and can be confusing for developers who are new to the code. In the future, we will probably add new attribute macros such as `#[derive_use_delegate]`, so that it can be used to derive `UseDelegate` implementations such as follows:

```rust
#[derive_use_delegate]
#[cgp_type {
    provider: ChainTypeProviderAt,
}]
pub trait HasChainTypeAt<I> {
    type Chain;
}
```

Until then, we have to manually implement `UseDelegate` with the same implementation pattern used in this chapter.
