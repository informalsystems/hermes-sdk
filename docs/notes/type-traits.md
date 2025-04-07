# Type Traits

Hermes SDK uses CGP type traits to define abstract types to be used with the relayer.

There are two patterns used in Hermes SDK when defining abstract types: a new approach and a legacy approach. Whenever possible, we should try to migrate away from the legacy pattern to use the new pattern.

## Naming Convention

Given an abstract type `X`:

- The consumer trait has the name `Has{X}Type`.
- The provider trait has the name `{X}TypeProvider`.
    - In legacy code, the provider trait has the name `Provide{X}Type`.
- The component type has the name `{X}TypeProviderComponent`.
    - In legacy code, the component type has the name `{X}TypeComponent`.

## Trait Items

- Whenever possible, an abstract type trait should contain exactly one associated type and no other associated methods in the trait.
- Having one associated type per trait would allow us to use the `#[cgp_type]` macro.

## Macro Use

- In new code, we use `#[cgp_type]` instead of `#[cgp_component]`, to define type components with generated type-related CGP constructs for us.
    - Whenever possible, we should migrate legacy type traits to use `#[cgp_type]`.
- The `#[cgp_type]` macro is an extension of the `#[cgp_component]` macro.
- `#[cgp_type]` can only be used if the trait contains exactly one associated type.
- The provider name is optional, and if unspecified will follow the convention `{X}TypeProvider`.

## `UseType`

- `#[cgp_type]` generates implementation of `UseType` and `WithProvider`, so that they can be used for simple wiring of concrete types.
- In legacy code, the implementation of `WithProvider` is defined manually. This can be migrated and removed when we switch to use `#[cgp_type]`.
- The new preferred way to wire concrete types is to use `UseType`.
    - In legacy wiring, `WithType` is used.
    - The type `WithType<X>` is simply an alias to `WithProvider<UseType<X>>`.
    - The use of `WithType` is functionally the same as `UseType`, but is more general and requires more advanced understanding of CGP, while the auto derivation of `UseType` can be understood more easily by anyone.
    - Since `#[cgp_type]` auto derives `UseType` for us, we can use `UseType` whenever possible to make the code simpler to understand.

## Example

Following is an example `Message` type trait:

```rust
#[cgp_type]
pub trait HasMessageType {
    type Message
}
```

We can "instantiate" the `Message` type to `MyMessage` by using `UseType` as follows:

```rust
pub enum MyMessage { ... }

#[cgp_context(MyChainComponents)]
pub struct MyChain { ... }

delegate_components! {
    MyChainComponents {
        MessageTypeComponent: UseType<MyMessage>,
    }
}
```
