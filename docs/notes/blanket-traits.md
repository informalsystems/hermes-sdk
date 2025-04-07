# Blanket Traits

The `#[blanket_trait]` macro is provided to simplify the implementation of traits that are meant to only contain blanket implementations and not implemented by others. Without the macro, we would first need to define the trait, and then define the blanket `impl` for that trait. But with `#[blanket_trait]`, the two definitions can be merged into one.

## Combining Multiple Traits

The first use of `#[blanket_trait]` is to define trait aliases that combine multiple traits into one.

For example, if we want to combine a `Foo` trait and a `Bar` trait together to become a `FooBar` trait, a manual approach would require the following definition:

```rust
pub trait FooBar: Foo + Bar {}

impl<Context> FooBar for Context
where
    Context: Foo + Bar,
{}
```

But with the use of `#[blanket_trait]`, the whole definition can be shorten to:

```rust
#[blanket_trait]
pub trait FooBar: Foo + Bar {}
```

Behind the scene, `#[blanket_trait]` generates a blanket implementation that can minimally satisfy the constraint of the trait definition.

## Blanket Methods

We can also use `#[blanket_trait]` to provide blanket implementations that contain methods, by implementing the given methods in the trait.

For example, suppose we want to implement a blanket implementation of `foobar()` that calls both `foo()` and `bar()`, a manual implementation would require the following to be written:


```rust
pub trait FooBar: Foo + Bar {
    fn foobar();
}

impl<Context> FooBar for Context
where
    Context: Foo + Bar,
{
    fn foobar(&self) {
        self.foo().bar()
    }
}
```

With `#[blanket_trait]`, the same definition can be shorten to:

```rust
#[blanket_trait]
pub trait FooBar: Foo + Bar {
    fn foobar(&self) {
        self.foo().bar()
    }
}
```

## Associated Type Alias

The use of `#[blanket_trait]` can allow us to also more easily define associated type aliases that can be used to simplify the reference to a given abstract type, especially when it is parameterized through a generic type.

For example, given the following type trait:

```rust
#[cgp_type]
pub trait HasChainTypeAt<I> {
    type Chain;
}
```

It can be quite tedious to refer to the `Chain` context at each index, such as `<Context as HasChainTypeAt<Index<0>>>::Chain` and `<Context as HasChainTypeAt<Index<1>>>::Chain`. We may want to define _associated type aliases_ `FirstChain` and `SecondChain`, so that we can refer to the first and second chains more easily in our code. A manual implementation of such alias would be as follows:

```rust
pub trait HasTwoChains:
    HasChainTypeAt<Index<0>, Chain = Self::FirstChain>
    + HasChainTypeAt<Index<1>, Chain = Self::SecondChain>
{
    type FirstChain;

    type SecondChain;
}

impl<Context, FirstChain, SecondChain>
    HasTwoChains for Context
where
    Context:
        HasChainTypeAt<Index<0>, Chain = FirstChain>
        + HasChainTypeAt<Index<1>, Chain = SecondChain>,
{
    type FirstChain = FirstChain;

    type SecondChain = SecondChain;
}
```

The way we understand how the associated type alias works is as follows:

- The trait `HasTwoChains` contains two associated types, `FirstChain` and `SecondChain`, which are unspecified at the trait definition.
- The trait has a supertrait `HasChainTypeAt<Index<0>, Chain = Self::FirstChain>`. This means that when the trait is used, the type `<Self as HasChainTypeAt<Index<0>>>::Chain` _must_ be the same as `Self::FirstChain`.
- Similarly, the supertrait `HasChainTypeAt<Index<0>, Chain = Self::FirstChain>` indicates that `<Self as HasChainTypeAt<Index<1>>>::Chain` must be the same as `Self::SecondChain`.
- In the blanket implementation, we introduce two generic parameters, `FirstChain` and `SecondChain`
- We bind `FirstChain` in `HasChainTypeAt<Index<0>, Chain = FirstChain>`, meaning that we now just refer to `<Context as HasChainTypeAt<Index<0>>>::Chain` as `FirstChain`.
- Similarly, with the binding of `SecondChain` in `HasChainTypeAt<Index<1>, Chain = SecondChain>`, it means that we now just refer to `<Context as HasChainTypeAt<Index<1>>>::Chain` as `SecondChain`.
- We then define the associated type `FirstChain` to be just `FirstChain`, and similarly for `SecondChain`.
- This means that effectively, we are just aliasing the associated types with new names.

In case if it is still confusing to you, the same blanket `impl` above can be rewritten as follows, which may be more verbose but perhaps less confusing:

```rust
impl<Context> HasTwoChains for Context
where
    Context:
        HasChainTypeAt<Index<0>>
        + HasChainTypeAt<Index<1>>,
{
    type FirstChain = <Context as HasChainTypeAt<Index<0>>>::Chain;

    type SecondChain = <Context as HasChainTypeAt<Index<1>>>::Chain;
}
```

Basically, the two implementations are equivalent, just written in different styles.

But the key takeaway is that with `#[blanket_trait]`, we do not need to hand write any of the blanket impls, and it would instead be automatically derived for us:


```rust
#[blanket_trait]
pub trait HasTwoChains:
    HasChainTypeAt<Index<0>, Chain = Self::FirstChain>
    + HasChainTypeAt<Index<1>, Chain = Self::SecondChain>
{
    type FirstChain;

    type SecondChain;
}
```

The main requirement for the use of associated type aliases is that the associated type must be bound to some other associated type inside the supertraits. In this case, we bind both `Self::FirstChain` and `Self::SecondChain` in the trait alias, and so `#[blanket_trait]` is able to generate the blanket implementation correctly for us.