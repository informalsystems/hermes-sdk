# Presets

Presets is an experimental feature of CGP to enable inheritance-like feature to group multiple providers together.

## `DelegateComponent` as Map

Conceptually, the `DelegateComponent` trait turns a type that implements it into something like a type-level key-value map. Consider the following example:

```rust
pub struct FooKey;
pub struct FooValue;
pub struct BarKey;
pub struct BarValue;

pub struct MappingA;

delegate_components! {
    MappingA {
        FooKey: FooValue,
        BarKey: BarValue,
    }
}
```

The struct `MappingA` uses `delegate_components!` to "set" entries into its map by implementing `DelegateComponents`. The macro call desugars into:

```rust
impl DelegateComponent<FooKey> for MappingA {
    type Delegate = FooValue;
}

impl DelegateComponent<BarKey> for MappingA {
    type Delegate = BarValue;
}
```

We can get a "value" of `MappingA`, such as `FooKey`, by referring to its associated type `<MappingA as DelegateComponent<FooKey>>::Delegate`, which will resolve to `FooValue`.

Once we can understand the use of `DelegateComponent` as table lookup, we can think of the generated blanket implementation of CGP components as performing "lookup" on the `DelegateComponent` entries. The component name type of each CGP component is used as the "key", to lookup for the provider "value" that implements a given a provider trait.

Of course, being a general-purpose type-level lookup table, we can also repurpose the use of `DelegateComponent` and `delegate_components!` to perform other kind of type-level mappings. We can see example uses of this in the use of `UseDelegate` and the encoding components.

## Extending Existing Mapping

With the coherence rules in Rust, the `DelegateComponent` trait can only be implemented by a mapping type owned by the crate. This means that a given mapping is "writable" within the owner crate, and then become "read-only" when accessed through other crates.

However, let's say `MappingA` is defined in crate `A`, but we want to extend it with a `BazKey` entry in crate `B`, now we cannot do that since crate `B` don't own `MappingA`. What we can do is to create a new mapping called `MappingB`, and make it extend from the existing entries from `MappingA` such as follows:

``` rust
pub struct BazKey,
pub struct BazValue,

pub struct MappingB;

delegate_components! {
    MappingB {
        FooKey: FooValue,
        BarKey: BarValue,
        BazKey: BazValue,
    }
}
```

With the naive approach above, we copy-pasted the foo and bar entries as defined in `MappingA`, and repeat them in `MappingB`. But because we have made copies, the entries can become out of sync if `MappingA` is updated. What if a different `FooValue2` is assigned to `FooKey` later on? What if a new `QuuxKey` entry is added to `MappingA`?

By analyzing the problem, we can quickly see that what we need is something like inheritence in this type-level lookup table. More specifically, we want to emulate something like prototypal inheritance in JavaScript, where we can "set" `MappingA` as the "prototype" of `MappingB`, so that we can use `MappingA` to lookup on any value that is not found on `MappingB`.

The problem here is that due to the coherence restrictions, we can't really emulate the inheritance behavior through generic implementations. A naive approach we can try would be to have a blanket implementation like:

```rust
impl<Key, Value> DelegateComponent<Key> for MappingB
where
    MappingA: DelegateComponent<Key, Delegate = Value>,
{
    type Delegate = Value;
}
```

But if we try defining that, we would quickly realize that we can no longer add `BazKey` to `MappingB`, if we don't also own `BazKey`. This is because Rust is overly conservative here, and reason that `MappingA` may one day decide to implement `DelegateComponent<BazKey>`, which then would in turn breaks `MappingB` if it also implemented `DelegateComponent<BazKey>`.

Unfortunately there is currently no way for us to tell Rust that we don't mind such breaking change from `MappingA`. So we cannot bypass Rust from preventing us to use this naive approach.

## `IsPreset` Trait

To workaround the earlier restriction, we need to somehow "tag" the key types, to trick Rust into "believing" that it is "safe" for us to implement `DelegateComponent` for a generic `Key` without potential upstream breakage. The way the trick work is by implementing a `IsPreset` trait in the same crate as we define `MappingA`:

```rust
pub trait IsPresetA<Key> {}

impl<T> IsPresetA<FooKey> for T {}
impl<T> IsPresetA<BarKey> for T {}
```

The `IsPresetA` trait is used here to "identify" that a given `Key` type "belongs" to `MappingA`. However, notice that the implementation of `IsPresetA` follows a blanket implementation for all type `T` with no additional condition. We will see next why we design and implement the trait this way.

With `IsPresetA`, we can now "safely" add a blanket implementation for `MappingB` as follows:

``` rust
pub struct BazKey,
pub struct BazValue,

pub struct MappingB;

impl<Key, Value> DelegateComponent<Key> for MappingB
where
    MappingA: DelegateComponent<Key, Delegate = Value>,
    Self: IsPresetA<Key>,
{
    type Delegate = Value;
}

delegate_components! {
    MappingB {
        BazKey: BazValue,
    }
}
```

Somehow, by having the constraint `Self: IsPresetA<Key>`, Rust now thinks that the crate we are in has control of which possible `Key` can be used for the generic implementation. Recall that because we implement `IsPresetA` with a blanket implementation for any `Self` type, it is also automatically implemented when `Self` is `MappingB`.

## `cgp_preset!` Macro

A primary feature provided by the `cgp_preset!` macro is to define a `IsPreset` trait for a component mapping, so that we can later use the trait to implement generic bulk delegation.

Using `cgp_preset!`, we can use it instead of `delegate_components!` to re-define `MappingA` as follows:

```rust
cgp_preset! {
    MappingA {
        FooKey: FooValue,
        BarKey: BarValue,
    }
}
```

An additional difference as compared to `delegate_components!` is that `cgp_preset!` defines the struct, together with a _module_ that wraps the component and the `IsPreset` trait inside. So the expansion would look something like:

```rust
pub mod MappingA {
    pub struct Provider;

    delegate_components! {
        Provider {
            FooKey: FooValue,
            BarKey: BarValue,
        }
    }

    pub trait IsPreset<Key> {}

    impl<T> IsPreset<FooKey> for T {}
    impl<T> IsPreset<BarKey> for T {}
}
```

There are also other things generated by `cgp_preset!`, which we will cover in later sections. The main takeaway here is that we now need to refer to the mapping as `MappingA::Provider`, and the preset trait as `MappingA::IsPreset`.