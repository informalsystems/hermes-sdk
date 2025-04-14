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

One minor tweak that we can improve is to delegate the value mapping back to `MappingA` itself, such as follows:

```rust
delegate_components! {
    MappingB {
        FooKey: MappingA,
        BarKey: MappingA,
        BazKey: BazValue,
    }
}
```

With this, in case if the values of `FooKey` or `BarKey` changed in `MappingA`, it will automatically tracked and we don't need to update `MappingB` manually.

It is worth noting that in the mapping above, we don't really map `FooKey` to the mapping of `<MappingA as DelegateComponent<FooKey>>::Value`, but we map to `MappingA` itself. This works for some cases, because what we need is not the value, but the _traits_ that value types such as `FooValue` and `BarValue` implements. For the cases such as CGP provider traits, the provider mappings such as `MappingA` would also have automatically implemented the provider trait through the delegation, so it is sufficient that we don't need to further "peek" into the value within `MappingA`.

However, when we use `delegate_components!` for mappings other than CGP providers, sometimes there are needs for us to "peek" further into the mapping, or even "do" something on the mapped value. A naive variant of such mapping would be something like:


```rust
delegate_components! {
    MappingB {
        FooKey: <MappingA as DelegateComponent<FooKey>>::Value,
        BarKey: <MappingA as DelegateComponent<BarKey>>::Value,
        BazKey: BazValue,
    }
}
```

But given that there are a lot of repetition, we can build some further abstractions to simplify the mapping. One common pattern is to use the `UseDelegate` pattern to perform the mapping, in addition to implementing a specific trait:

```rust
delegate_components! {
    MappingB {
        FooKey: UseDelegate<MappingA>,
        BarKey: UseDelegate<MappingA>,
        BazKey: BazValue,
    }
}
```

or if we use the array notation to group common keys, it would further simplify to become:

```rust

delegate_components! {
    MappingB {
        [
            FooKey,
            BarKey,
        ]: UseDelegate<MappingA>,
        BazKey: BazValue,
    }
}
```

Ultimately, whether the mapping should be done on `MappingA`, `UseDelegate<MappingA>`, or `<MappingA as DelegateComponent<FooKey>>::Value`, depends on the specific use cases, which are worth going into further details in other chapters. For the purpose here, the rule of thumb is to use `MappingA` when we are wiring providers, and to use `UseDelegate<MappingA>` when we are mapping other things, such as mapping of error handlers, encoders, and counterparty-specific methods.


## Inheritance

By analyzing the problems earlier, we can quickly see that what we need is something like inheritence in this type-level lookup table. More specifically, we want to emulate something like prototypal inheritance in JavaScript, where we can "set" `MappingA` as the "prototype" of `MappingB`, so that we can use `MappingA` to lookup on any value that is not found on `MappingB`.

The problem here is that due to the coherence restrictions, we can't really emulate the inheritance behavior through generic implementations. A naive approach we can try would be to have a blanket implementation like:

```rust
impl<Key> DelegateComponent<Key> for MappingB
where
    MappingA: DelegateComponent<Key>,
{
    type Delegate = MappingA;
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

impl<Key> DelegateComponent<Key> for MappingB
where
    Self: IsPresetA<Key>,
{
    type Delegate = MappingA;
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

## `#[cgp_context]` Macro

The `#[cgp_context]` macro supports for single inheritance of a context's provider with a context-generic preset that is defined elsewhere. Behind the scene, it makes use of the `IsPreset` trait to perform bulk delegation onto the given preset. For example, given the following code:

```rust
#[cgp_context(MyChainComponents: MyChainPreset)]
pub struct MyChain;
```

the macro would generate the following code:

```rust
pub struct MyChainComponents;

impl HasProvider for MyChain {
    type Provider = MyChainComponents;
}

impl<Name> DelegateComponent<Name> for MyChainComponents
where
    Self: MyChainPreset::IsPreset<Name>,
{
    type Delegate = MyChainPreset::Provider;
}
```

With this, we are able to separate a collection of providers into presets, and then "inherit" from them when we define the concrete contexts.

## Multiple Inheritance

The use of `IsPreset` for single inheritance only works if we inherit a final provider with a preset. However, it is not possible to define one preset that "inherits" from another preset, or combine the use of multiple `IsPreset` traits.

Similarly, we cannot use the `IsPreset` trait to perform inheritance over multiple presets. This is because we can only have one blanket implementation of `DelegateComponent` with generic key. So even the use of `IsPreset` cannot help us workaround here, even if Rust can statically determine that there will be no overlap.

However, having some kind of multiple inheritance would still be useful for CGP, since CGP presets are similar collection of mixins that can be mixed and matched. The way we solve this currently is by using macros to syntactically "copy" over the list of keys to be expanded at the target site.

## `Preset::with_components!` Macro

When using `cgp_preset!`, it also generates a `with_components!` macro that would capture all the keys in a mapping, and then expand it at the call site. We can for example use it to extend the mapping for the earlier `MappingB` as follows:

```rust
MappingA::with_components! {
    | Keys | {
        delegate_components! {
            MappingB {
                Keys: MappingA::Provider,
                BazKey: BazValue,
            }
        }
    }
}
```

The macro works in a continuation-passing-style with a closure-like syntax. It will replace the identifier `Keys` in the macro argument, with all the keys defined in `MappingA`, i.e. `[FooKey, BarKey]`. So after the macro expansion, the code becomes the same as before:

```rust
delegate_components! {
    MappingB {
        [
            FooKey,
            BarKey,
        ]: MappingA::Provider,
        BazKey: BazValue,
    }
}
```

## `#[re_export_imports]` Macro

The use of `with_components!` macro is a hackish way of getting multiple inheritance work. However, a key issue is that the macro can only capture and expand syntax _tokens_, but not the actual types or where they were imported from. This can be a problem, because we would then have to manually import `FooKey` and `BarKey` when defining `MappingB` for the macro to work. This would not only be tedious, but also poorly integrated with IDEs like Rust Analyzer, which cannot easily provide auto fix on errors that arise from macro expansions.

What we need is to come up with yet another hack, so that when defining the `MappingA` preset, we also re-export _everything_ that we have imported over there. As a result, we also need to create a dummy module and use `#[cgp::re_export_imports]` to capture and re-export all the imports within that module.

A full definition of `MappingA` with re-exports would be something like follows:

```rust
// Inside a file like presets/mapping-a.rs

#[cgp::re_export_import]
mod preset {
    use cgp::prelude::*;

    use crate::foo::{FooKey, FooValue};
    use crate::bar::{BarKey, BarValue};

    cgp_preset! {
        MappingA {
            FooKey: FooValue,
            BarKey: BarValue,
        }
    }
}
```

The outer `#[cgp::re_export_imports]` macro would then expand the module into something like:

```rust
mod preset {
    use cgp::prelude::*;

    use crate::foo::{FooKey, FooValue};
    use crate::bar::{BarKey, BarValue};

    cgp_preset! {
        MappingA {
            FooKey: FooValue,
            BarKey: BarValue,
        }
    }

    mod re_exports {
        pub use cgp::prelude::*;
        pub use crate::foo::{FooKey, FooValue};
        pub use crate::bar::{BarKey, BarValue};
    }
}

pub use preset::*;
```

Essentially, `#[cgp::re_export_imports]` creates an inner `re_exports` module, and repeat all the import statements of the parent module with an additional `pub` keyword to make them re-exports. It is also worth noticing that the macro generates a `pub use preset::*;` at the end, so we can import and use the module content as if the inner module don't exist.

Within the `cgp_preset!` macro, it also requires a `re_exports` module to be present at the same scope, so that it can re-export them again inside `Preset::re_exports`.

With this additional logistics in place, we just need to add one more line to automatically import all re-exports from `MappingA` inside `MappingB`:


```rust
use MappingA::re_exports::*;

MappingA::with_components! {
    | Keys | {
        delegate_components! {
            MappingB {
                Keys: MappingA::Provider,
                BazKey: BazValue,
            }
        }
    }
}
```

## Filtering Keys

When inheriting from presets, we sometimes want to filter out some of the keys, so that we can override them with some other values. This may also be useful in solving the "diamond inheritance" problem, which would just result in errors on overlapping instances in Rust.

The `Preset::with_components!` macro accepts an optional list of key filters to exclude the keys from the macro expansion. For example, we can write:

```rust
MappingA::with_components! {
    [
        BarKey,
    ],
    | Keys | {
        delegate_components! {
            MappingB {
                Keys: MappingA::Provider,
                BarKey: NewBarValue,
                BazKey: BazValue,
            }
        }
    }
}
```

In this case, the key `BarKey` will be filtered out, and the `Keys` "variable" would be expanded into just `[FooKey]`. With that, we can for example replace the entry with a new value such as `NewBarValue`. The end result expansion becomes:

```rust
delegate_components! {
    MappingB {
        [
            FooKey,
        ]: MappingA::Provider,
        BarKey: NewBarValue,
        BazKey: BazValue,
    }
}
```

It is worth noting that the filtering works only on the _syntactic identifier_ of the original map. So if `MappingA` was doing some import renaming on the keys, it may not get recognized when the original key name is given in the filter list.

## Future Improvements

The current iteration of the preset macros are better than before, but is still not very elegant. It is possible that in future versions of CGP, we will improve the usability with new macros and syntaxes.

At the time of writing, this feature is highly unstable and may be subject to breaking changes. Until then, I hope that this document will help make navigating the presets and inheritance a little easier.
