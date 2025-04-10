# Getter Traits

Hermes SDK uses CGP getter (accessor) traits to define getter methods to access fields in a context.

There are two patterns used in Hermes SDK when defining getter traits: a new approach and a legacy approach. Whenever possible, we should try to migrate away from the legacy pattern to use the new pattern.

## Naming Convention

Given a getter for a field `x`:

- The consumer trait has the name `Has{X}`.
- The provider trait has the name `{X}Getter`.
- The component type has the name `{X}GetterComponent`.

## Trait Items

- Whenever possible, a getter trait should contain exactly one accessor method.
- In legacy code, we may also see getter traits with more than one getter methods. They are now being discouraged, and should be refactored into multiple smaller getter traits.
- As explained later, having one getter method per trait would allow the use of `UseField` for simple wiring of getter methods to fields in a concrete context.

### Macro Use

- In the new code, we use `#[cgp_getter]` instead of `#[cgp_component]` to define getter components with generated getter-related CGP constructs for us.
    - Whenever possible, we should migrate legacy getter traits to use `#[cgp_getter]`.
- The `#[cgp_getter]` macro is an extension of `#[cgp_component]`, with additional derived constructs.


### `UseField`

- When `#[cgp_getter]` is used with traits with only one getter method, it would generate `UseField` and `WithProvider` implementations that can be used for simple wiring of field access.
- `UseField` can be used to wire up a getter method with a specific field in the context.
    - The name of the field is specified using the `symbol!` macro to turn a string into a type parameter.
    - The context is required to use `#[derive(HasField)]` to derive `HasField` implementations that will be used by `UseField` to implement the getter method.
- In legacy code, the implementation of `withProvider` is defined manually. This can be migrated and removed when we switch to use `#[cgp_type]`.
- The new preferred way to wire getter is to use `UseField`.
    - In legacy wiring, `WithField<X>` is used, which is simply an alias to `WithProvider<UseField<X>>`.
    - The use of `WithField` is functionally the same as `UseField`, but is more general and requires more advanced understanding of CGP, while the auto derivation of `UseField` can be understood more easily.
    - Since `#[cgp_getter]` auto derives `UseField` for us, we can use `UseField` whenever possible to make the code simpler to understand.

## Example

Following is an example `RpcClient` getter trait:

```rust
#[cgp_getter {
    provider: RpcClientGetter,
}]
pub trait HasRpcClient {
    fn rpc_client(&self) -> &RpcClient;
}
```

We can wire the RPC client getter with a `rpc_client` field in a context as follows:

```rust
#[cgp_context(MyChainComponents)]
#[derive(HasField)]
pub struct MyChain {
    pub rpc_client: RpcClient,
    ...
}

delegate_components! {
    MyChainComponents {
        RpcClientGetterComponent:
            UseField<symbol!("rpc_client")>,
        ...
    }
}
```

## Getter with Abstract Types

- A getter trait can also be used with abstract type trait, so that it can be used to access fields with abstract types.
- We can just include the type trait as a supertrait to the getter trait.

Following is an example RPC client getter with abstract `RpcClient` type:

```rust
#[cgp_type]
pub trait HasRpcClientType {
    type RpcClient;
}

#[cgp_getter {
    provider: RpcClientGetter,
}]
pub trait HasRpcClient: HasRpcClientType {
    fn rpc_client(&self) -> &Self::RpcClient;
}
```

To wire up the getter, we now need to wire both the type trait and the getter trait:

```rust
#[cgp_context(MyChainComponents)]
#[derive(HasField)]
pub struct MyChain {
    pub rpc_client: JsonRpcClient,
    ...
}

delegate_components! {
    MyChainComponents {
        RpcClientTypeProviderComponent:
            UseType<JsonRpcClient>,
        RpcClientGetterComponent:
            UseField<symbol!("rpc_client")>,
        ...
    }
}
```

## Getter with Generic Parameter

- A getter trait can contain generic parameters to multiplex into multiple getter methods to access separate fields of the same type in a context.
- When defining such getter trait, the getter method can accept an additional `PhantomData` parameter to assist the Rust compiler to perform type inference on the code.
- We can also add the generic parameter to the component type, so that we can apply different wiring for each generic type.

For example, the following `HasWallet` trait allows the context to provide more than one wallets, as tagged by a generic type `I`:

```rust
#[cgp_getter {
    name: WalletGetterComponent<I>,
    provider: WalletGetter,
}]
pub trait HasWallet<I> {
    fn wallet(&self, _tag: PhantomData<I>) -> &Wallet;
}
```

A concrete context can then choose how many wallets it want to provide. The example below defines a concrete context with two wallets, identified by `Index<0>` and `Index<1>`:

```rust
#[cgp_context(MyChainComponents)]
#[derive(HasField)]
pub struct MyChain {
    pub wallet_a: Wallet,
    pub wallet_b: Wallet,
}

delegate_components! {
    MyChainComponents {
        WalletGetterComponent<Index<0>>:
            UseField<symbol!("wallet_a")>,
        WalletGetterComponent<Index<1>>:
            UseField<symbol!("wallet_b")>,
    }
}
```


## Auto Getters

- The use of `#[cgp_getter]` and `UseField` provides flexibility in choosing which field in the concrete context can be used to implement a getter trait.
    - It also allows contexts to implement a getter method manually, such as by returning a hard-coded value.
- However, we sometimes don't need such flexibility, and instead just want a certain hard-coded field to exist in a context.
- In such cases, we can use `#[cgp_auto_getter]`, which is automatically implemented for all contexts with the given fields, and do not require further wiring using `UseField`.
- The use of `#[cgp_auto_getter]` can be convenient, especially during early prototyping, since it require less boilerplate.
- When the need arise later, we can easily migrate a trait away from `#[cgp_auto_getter]` to use `#[cgp_getter]`.

For example, the `HasRpcClient` trait can be re-defined as follows:

```rust
#[cgp_auto_getter]
pub trait HasRpcClient {
    fn rpc_client(&self) -> &RpcClient;
}
```

With that, a context only needs to derive `HasField`, and would automatically implement `HasRpcClient` if it contains the field with the same name as the getter method:

```rust
#[derive(HasField)]
pub struct MyChain {
    pub rpc_client: RpcClient,
    ...
}
```