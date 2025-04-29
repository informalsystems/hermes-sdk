# Context Fields

This document provides some guidelines on how to define the fields in a concrete context.

## Plain Fields

When getting started, the most straightforward way to define the context fields is to add them directly inside the concrete context struct:

```rust
#[cgp_context(MyContextComponents)]
#[derive(HasField, Clone)]
pub struct MyContext {
    pub config: Config,
    pub runtime: Runtime,
    pub other_fields: OtherFields,
    ...
}
```

Typically, a concrete context would derive `HasField`, so that we can use providers like `UseField` to wire up field accessors. In Hermes SDK, it is also common for the context to derive `Clone`, so that we can clone a context to use it inside multiple async tasks or parent contexts.

However, the auto-derived `Clone` implementation may be inefficient if the context contains many fields. If so, the cloning could become expensive quickly if it is done many times.

## `Arc` Fields

A more efficient approach is to hide the context fields behind an `Arc`, so that we can clone the context more cheaply:

```rust
#[cgp_context(MyContextComponents)]
pub struct MyContext {
    pub fields: Arc<MyContextFields>,
}

#[derive(HasField)]
pub struct MyContextFields {
    pub config: Config,
    pub runtime: Runtime,
    pub other_fields: OtherFields,
    ...
}

impl Deref for MyContext {
    type Target = MyContextFields;

    fn deref(&self) -> &MyContextFields {
        &self.fields
    }
}
```

In the above example, we move all fields of `MyContext` into a separate struct `MyContextFields`. The original context now contains an `Arc<MyContextFields>` field, which can now be cloned cheaply.

We also implement `Deref` for `MyContext` with `MyContextFields` being the target. This way, we can use the fields in `MyContextFields` directly as if they are defined in `MyContext`.

We also move the derivation of `HasField` from `MyContext` to `MyContextFields`. The `HasField` trait has a blanket implementation for types that implement `Deref`. So `MyContext` automatically implements any `HasField` that is implemented by `MyContextFields`.

## Fields with Abstract Types

As we make more advanced uses of CGP, we may want to make use of CGP traits to define fields with _abstract types_ inside a context, using the component wiring of that context.

For example, we may want to define the `runtime` field with the abstract `Runtime` type as follows:

```rust
#[cgp_context(MyContextComponents)]
#[derive(HasField, Clone)]
pub struct MyContext {
    pub runtime: <MyContext as HasRuntimeType>::Runtime,
    ...
}

delegate_components! {
    MyContextComponents {
        RuntimeTypeProviderComponent: UseType<MyRuntime>,
        ...
    }
}
```

In the above example, we have a wiring of `RuntimeTypeProviderComponent` to use the type `MyRuntime` as the runtime. So conceptually, the field `MyContext::runtime` would resolve to have the type `MyRuntime`.

However, if we try to compile the code, we would encounter cyclic error that we try to resolve the trait implementation of `MyContext` before the struct is fully defined. As a result, we would need different workaround to be able to use abstract types that refer back to the context itself.

For the above example, it may seem trivial that we can replace the field definition directly with `MyRuntime`. However, there are more complex examples where the context definition can be significantly simplified using composite abstract types that are made of multiple inner abstract types. An example would be the `MessageBatchSender` type for the relay context, which is made of a combination of sender, receiver, message, event, and error.

A naive attempt to solve this would be to use the context fields pattern earlier, and make use of the abstract field type inside `MyContextFields`:

```rust
#[cgp_context(MyContextComponents)]
pub struct MyContext {
    pub fields: Arc<MyContextFields>,
}

#[derive(HasField)]
pub struct MyContextFields {
    pub runtime: <MyContext as HasRuntimeType>::Runtime,
    ...
}

impl Deref for MyContext {
    type Target = MyContextFields;

    fn deref(&self) -> &MyContextFields {
        &self.fields
    }
}

delegate_components! {
    MyContextComponents {
        RuntimeTypeProviderComponent: UseType<MyRuntime>,
        ...
    }
}
```

However, if we try to compile the code, we would still encounter the same error. Essentially, to define `MyContext`, we need to define `MyContextFields`, which needs `MyContext` to be defined first, resulting in an infinite loop.

## Proxy Field Traits

The solution is to define a _proxy field trait_ that "hides" the concrete field type behind a trait.


```rust
#[cgp_context(MyContextComponents)]
pub struct MyContext {
    pub fields: Arc<dyn HasMyContextFields>,
}

#[derive(HasField)]
pub struct MyContextFields {
    pub runtime: <MyContext as HasRuntimeType>::Runtime,
    ...
}

pub trait HasMyContextFields {
    fn fields(&self) -> &MyContextFields;
}

impl HasMyContextFields for MyContextFields {
    fn fields(&self) -> &MyContextFields {
        self
    }
}

impl Deref for MyContext {
    type Target = MyContextFields;

    fn deref(&self) -> &MyContextFields {
        &self.fields.fields()
    }
}

delegate_components! {
    MyContextComponents {
        RuntimeTypeProviderComponent: UseType<MyRuntime>,
        ...
    }
}
```

In the above example, we introduce a `HasMyContextFields` trait, with an accessor method to convert a `&self` to `&MyContextFields`. We then trivially implement `HasMyContextFields` for `MyContextFields`, with it just returning `self`.

In the definition of `MyContext`, we now define the `fields` to be `Arc<dyn HasMyContextFields>`. Now even though it is essentially the same as `Arc<MyContextFields>`, the `dyn` trait makes Rust "forgets" the concrete type, and allows the struct `MyContext` to be considered defined without trying to walk into `MyContextFields`.

With this technique in place, we are able to get around the cyclic error, and define our context struct with abstract field types directly.
