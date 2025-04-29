# Code Navigation

This document provides a quick guide on how to navigate the code base in Hermes SDK.

With CGP being new to most people, one can easily get lost in figuring how to start navigating the code base in Hermes SDK. The main challenge lies in knowing which part of the code to look for, and how to find out which concrete implementation is being used. However, with some quick guidance here, you can learn that the navigation is concentrated at a few places, and it would become much clearer once you know where to look.

## Start with concrete contexts

When getting started with Hermes SDK, the main goal of a new developer would typically be figuring out the execution path of a concrete implementation of the relayer, such as the Cosmos relayer. To do that, it would be useful to start looking at crates that contain the concrete contexts, such as `hermes-cosmos-relayer`.

From the crate, you can start looking at how concrete contexts such as `CosmosChain` is defined. The first thing to look for is the use of `#[cgp_context]` attribute, such as:

```rust
#[cgp_context(CosmosChainContextComponents: CosmosChainPreset)]
pub struct CosmosChain {
    ...
}
```

The `#[cgp_context]` macro helps with performing the wiring of the implementation of consumer traits for `CosmosChain` with the provider `CosmosChainContextComponents`. Often, the given provider also inherits from another preset, such as `CosmosChainPreset`, so that the bulk of the wiring can be reused by multiple concrete contexts.

## Look for `delegate_components!`

Once we know the context provider that is wired with the concrete context, we can next look for `delegate_components!` calls that perform the wiring of individual providers with the context provider. It would usually be defined at the same file as the concrete context, and looks something like:

```rust
delegate_components! {
    CosmosChainContextComponents {
        ...
    }
}
```

Sometimes, a context provider may also directly implement a provider trait specifically for the concrete context. This is typically done in the same file. So you should also look for implementation blocks such as:

```rust
#[cgp_provider(GrpcAddressGetterComponent)]
impl GrpcAddressGetter<CosmosChain> for CosmosChainContextComponents {
    ...
}
```

## Look for `cgp_preset!`

When a concrete context is simple, we can find all the wirings to be done within the context provider. But for more complicated contexts, we would also need to look for the wiring of the preset that it is inherited from, which would be defined with the `cgp_preset!` macro:

```rust
cgp_preset! {
    CosmosChainPreset {
        ...
    }
}
```

The preset is usually defined in a separate module or crate. But you can use Rust Analyzer to hover over the identifier used in `#[cgp_context]`, and jump to the definition quickly.

The rules for whether to look for a wiring in the base context provider or the inherited preset is similar to how inheritance works in OOP. So we know that we can start from looking at the context provider, and then only look for the wiring at the preset if one is not found.

Aside from the base inheritance, a preset may also inherit from a parent preset with the `ParentPreset::with_components!` macro. In such cases, we just need to look up to the ancestor presets, all the way until we find the component wiring that we look for.

## Look for check traits

The wirings of providers in a concrete context is done declaratively, with the concrete implementation instantiated lazily when it is first used. When looking just at the wirings, it may not be clear what has been implemented, and whether the wiring is correct. A good place to look at next is the use of `check_components!`, which would check on whether the concrete context implements a certain component.

```rust
check_components! {
    CanUseCosmosChainComponents for CosmosChain {
        ...
    }
}
```

Currently, the use of `check_components!` is not very comprehensive. So if you are unsure and wanted to check whether the context have implemented a certain component, it is a good idea to quickly add an additional line in `check_components` to check on the wiring.

## Watch out for `UseDelegate` dispatches

Another main source of confusion for newcomers are when encountering the use of `UseDelegate` to implement static dispatches. The `UseDelegate` pattern is used to dispatch on the provider implementations, beased on the extra generic parameters of consumer traits. For example, chain contexts like `CosmosChain` use it to dispatch chain implementations based on the counterparty chain.

As a result, it is good to know what to look next when encountering delegation to `UseDelegate` such as:

```rust
cgp_preset! {
    CosmosChainPreset {
        ...
        [
            ...
            ClientStateQuerierComponent,
            ...
        ]:
            UseDelegate<DelegateCosmosChainComponents>,
    }
}
```

The key to understand here is that the implementation of `ClientStateQuerierComponent` depends on which `Counterparty` type is used with the main chain context. The wiring look up is now forwarded to the `DelegateCosmosChainComponents` type. So we need to look for an implementation of `impl DelegateComponent<Counterparty> for DelegateCosmosChainComponents`.

For the case of Cosmos to Cosmos relayer, we know that we want to look up for an implementation of of `ClientStateQuerierComponent` for `CosmosChain`, where the counterparty is _also_ `CosmosChain`. So we look around and find the entry:

```rust
delegate_components! {
    DelegateCosmosChainComponents {
        CosmosChain: CosmosToCosmosComponents::Provider,
    }
}
```

So we know that when `Counterparty` is `CosmosChain`, then the implementation is forwarded to `CosmosToCosmosComponents::Provider`, which contains the entry:

```rust
    cgp_preset! {
        CosmosToCosmosComponents {
            ...
            ClientStateQuerierComponent: QueryAndConvertRawClientState,
            ...
        }
    }
```

So with that, we reached the actual provider implementation for `ClientStateQuerier<CosmosChain, CosmosChain>`, which is `QueryAndConvertRawClientState`.

## Example Walkthrough

We will have an example walk through to demonstrate how to navigate the code for Hermes SDK.

Supposed that we want to find out how the relayer starts auto-relaying packets between two Cosmos chains. We first need to identify the trait that is called, which in our case is `CanAutoBiRelay`.

We then need to find the concrete context which contains the wiring of the component `AutoBiRelayerComponent`, which in this case is `CosmosBiRelay`:

```rust
#[cgp_context(CosmosBiRelayComponents: DefaultBiRelayComponents)]
pub struct CosmosBiRelay {
    ...
}
```

With the `#[cgp_context]` definition, we know to first look for a wiring of `AutoBiRelayerComponent` in `CosmosBiRelayComponents`, which we don't find. we then look one level up for the wiring at the `DefaultBiRelayComponents` preset, and found it as:

```rust
cgp_preset! {
    DefaultBiRelayComponents {
        ...
        AutoBiRelayerComponent: PerformAutoBiRelay,
    }
}
```

With that, we know that `CosmosBiRelay` uses `PerformAutoBiRelay` to implement `AutoBiRelayerComponent`.

Inside the implementation of `PerformAutoBiRelay`, we see that `CanAutoRelayWithHeights` is used to perform the inner auto relaying on the relay contexts `RelayAToB` and `RelayBToA`. We now need to find out which concrete relay contexts are wired with `CosmosBiRelay`, which we can find out to be `CosmosRelay` in the wiring in `CosmosBiRelayComponents`:

```rust

delegate_components! {
    CosmosBiRelayComponents {
        ...
        [
            RelayTypeProviderAtComponent<Index<0>, Index<1>>,
            RelayTypeProviderAtComponent<Index<1>, Index<0>>,
        ]: UseType<CosmosRelay>,
    }
}
```

So we then look at the context definition of `CosmosRelay`:

```rust
#[cgp_context(CosmosRelayComponents: ExtraRelayPreset)]
pub struct CosmosRelay {
    ...
}
```

In this case, we find that `AutoRelayerWithHeightsComponent` is not found in `CosmosRelayComponents` and `ExtraRelayPreset`. But `ExtraRelayPreset` is inherited from `DefaultRelayPreset` with the definition:

```rust
DefaultRelayPreset::with_components! {
    ...
    | Components | {
        cgp_preset! {
            ExtraRelayPreset {
                ...
                Components:
                    DefaultRelayPreset::Provider,
            }
        }
    }
}
```

So we look into `DefaultRelayPreset` and found the wiring to be `RelayWithPolledEvents`:

```rust
cgp_preset! {
    DefaultRelayPreset {
        AutoRelayerWithHeightsComponent: RelayWithPolledEvents,
    }
}
```

With that, we can now look at the implementation of `RelayWithPolledEvents`, and try and understand how it implements `AutoRelayerWithHeights<CosmosRelay, SourceTarget>` and `AutoRelayerWithHeights<CosmosRelay, DestinationTarget>`.

Following similar steps, we can further trace into how each component is wired and implemented in the contexts, all the way until the final implementation is reached.

Hopefully with this walk through, it is now clearer on how you can navigate the Hermes SDK code base.