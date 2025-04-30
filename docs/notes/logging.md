# Logging

This document gives a quick overview of how logging is done in Hermes SDK.

## Motivation

In Hermes SDK, we try to decouple the concrete logging implementation as much as possible from `tracing`, especially in the abstract core relaying logic.

The main reason for this decoupling is that Tracing is a very heavyweight library, and we may not always want to use it in certain contexts, such as when verifying for correctness in Kani or running in WebAssembly. Originally, we also tried to migrate to more light weight logging libraries that support structured logging, such as [slog](https://github.com/slog-rs/slog).

However, with the limited future plans for Hermes SDK, this requirement may be relaxed, since we are almost always using Tracing for now. Nevertheless, it is highly recommended to follow the CGP principles, at least for the core relaying logic in crates such as `hermes-relayer-components`.

## Abstract Logging

When implementing abstract components, in Hermes SDK we make use of a minimal `CanLog` trait to support abstract but extensible logging. The trait is defined as follows:

```rust
#[cgp_component {
    provider: Logger,
}]
#[async_trait]
pub trait CanLog<Details>: Async
where
    Details: Send + Sync,
{
    async fn log(&self, message: &str, details: &Details);
}
```

The `CanLog` trait is parameterized by a generic `Details` type, which can contain arbitrary metadata that the caller can provide. The `log` method also accepts a general `message` string, in addition to `Details`. The message string is meant to be informative only, as it is not productive to include additional metadata inside strings. Instead, any additional metadata is encapsulated inside the `Details` type.

## Example

Following is an example use of `CanLog` with a custom `Details` type:

```rust
pub struct GreetDetails<'a> {
    pub name: &str,
}

#[cgp_component {
    provider: Greeter,
}]
pub trait CanGreet {
    fn greet(&self);
}

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_new_provider(GreeterComponent)]
impl<Context> Greeter<Context> for LogHello
where
    Context: HasName + for <'a> CanLog<GreetDetails<'a>>,
{
    fn greet(context: &Context) {
        context.log("Hello!", &GreetDetails {
            name: context.name(),
        });
    }
}
```

We define a `GreetDetails` struct that contains metadata that we want to log, which in our case is the name string. Inside the example provider `LogHello`, we call `context.log()` with the message `"Hello!"` and the name contained inside `GreetDetails`.

At this point, you may wonder how do the logger implementation actually logs `GreetDetails`. But that is an implementation detail that we will discussed later. With CGP, the consumer and the provider are fully decoupled, so the logging consumer can require the context to log any kind of `Details`.

## Log Levels

One thing that is noticeably missing inside the `log` method is the ability to specify the log level. This is because strictly speaking, the precise log level that should be used is depending on the concrete application/context.

Instead, the caller of `CanLog` can depend on the specific logging implementation to _choose_ which log level to log the message, depending on the `Details` type.

However, Hermes SDK also provide general `Details` types, named by the log levels, as a simplified way to log generic log messages that do not contain metadata. These types are `LevelTrace`, `LevelDebug`, `LevelInfo`, `LevelWarn`, and `LevelError`. It is worth noting that these level types are just empty struct, and they are more like a tag that distinguish them as distinct `Details` types.

Using the generic log levels, we can for example quickly log with `LevelInfo` without defining custom `Details` structs:

```rust
#[cgp_new_provider(GreetComponent)]
impl<Context> Greeter<Context> for LogHello
where
    Context: HasName + CanLog<LevelInfo>,
{
    fn greet(context: &Context) {
        context.log(&format!("Hello, {}!", context.name()), &LevelInfo);
    }
}
```

Compared to before, the `Greeter` implementation here skips defining the `GreetDetails` struct, and instead uses the general `LevelInfo` details. But since `LevelInfo` is an empty struct, this also means that we cannot attach additional metadata in there. Instead, the implementataion now calls `log` with a formatted string, with the name embedded within the string.

This approach of using general log levels can be convenient for quick logging. However as we can see, this could make metadata extraction more challenging. Depending on the situation, it may be feasible to start with logging with general levels, and then switch to custom `Details` types when more fine grained logging is required for that specific entry.

It is also worth noting that the level types such as `LevelInfo` are purely informative only. A logging implementation may choose to log in a different level, or omit the log level completely. This could for example be useful to remove logging code at compile time.

## Logging Providers

With the log interface in place, we can now implement logging for specific `Details` type. For example:

```rust
#[cgp_new_provider(LoggerComponent)]
impl<'a, Context> Logger<Context, GreetDetails<'a>> for LogGreetWithTracing {
    fn log(&self, message: &str, details: &GreetDetails<'a>) {
        tracing::info!(
            name = %details.name,
            "{message}",
        )
    }
}

#[cgp_new_provider(LoggerComponent)]
impl<'a, Context> Logger<Context, LevelInfo> for LogInfoWithTracing {
    fn log(&self, message: &str, details: &LevelInfo) {
        tracing::info!(
            "{message}",
        )
    }
}
```

The above example shows two separate `Logger` providers, `LogGreetWithTracing` and `LogInfoWithTracing`. `LogGreetWithTracing` specifically knows how to log `GreetDetails`, by using `tracing::info!` to format the name details. `LogInfoWithTracing` specifically knows how to log `LevelInfo`, by simply logging the message with `tracing::info!`

From the example, we can see that it is possible to have multiple `Logger` providers that only know how to handle specific `Details` types. However, we can use `UseDelegate` to combine these providers together, to build a combined `Logger` provider that knows how to log all `Details` type that are needed for a concrete context. For example:

```rust
pub struct MyLoggers;

delegate_components! {
    MyLoggers {
        for<'a> GreetDetails<'a>: LogGreetWithTracing,
        LevelInfo: LogInfoWithTracing,
    }
}

#[cgp_context(MyContextComponents)]
#[derive(HasFields)]
pub struct MyContext {
    pub name: String,
}

delegate_components! {
    MyContextComponents {
        GreeterComponent: LogHello,
        LoggerComponent: UseDelegate<MyLoggers>,
    }
}
```

In the above example, we define a mapping struct `MyLoggers` to map from the concrete `Details` type to the specific `Logger` provider that we want to use. Then inside the main wiring for `MyContextComponents`, we make use of `UseDelegate<MyLoggers>` to dispatch the logging implementation based on the `Details` type.

## Tracing Logging Components

In Hermes SDK, the `hermes-tracing-logging-components` crate is provided with a `TracingLogger` provider that implements all common `Details` types that are used inside the abstract relayer crates. If a concrete context do not contain wirings that introduce new `Details` types, then the `TracingLogger` provider can be wired directly with `LoggerComponent`.

Otherwise, it may be necessary to make use of the `UseDelegate` pattern as we discussed in the previous section, so that we can dispatch between the custom loggers and the default `TracingLogger`.

Alternatively, one may add an implementation of `Logger` directly on `TracingLogger` inside the `hermes-tracing-logging-components` crate. However, this should only be done if doing so does not pull in extra concrete dependencies, as this could affect all other crates that import `hermes-tracing-logging-components`.

When working on chain-specific implementations, one could also opt in to just calling `tracing` directly, or use the general log level types to avoid adding the extra wirings.

## Structured Logging

One potential enhancement for the logging implementation is that in principle, we can implement a generic `Logger` provider that could work on _all_ generic `Details` type that implements `HasFields`, similar to the encoding providers. This way, new `Details` type could be introduced anywhere, and automatically get supported by the generic logger.

However, a major limitation that prevents us from doing so is that `tracing` does not have good support for structured logging at runtime. Currently, the only way to do structured logging in Tracing is by specifying them as separate fields inside the tracing macros. But since generic code cannot parameterize macros by the generic parameters, we instead need ways to call the log methods by passing the fields as function parameters.

Although Tracing do have plans to support structured logging through the [valuable](https://docs.rs/tracing/latest/tracing/#unstable-features) crate, the feature has remained unstable for a very long time. So as long as we are stuck with Tracing, it is not possible to have more generic logger implementations that can work for generic `Details` types.

## Future Considerations

Depending on the future directions, it may be possible for Hermes SDK to make more use of the abstract `CanLog` method, or more use of the concrete `tracing` methods directly. Both approaches are equally feasible, depending on the preferences of the developers.

Nevertheless, it is also important to note that even though the Hermes SDK core uses `CanLog`, there is no restriction on how external projects that use Hermes SDK should organize the logging. So a crate like `starknet-chain-components` is free to choose to log by calling `tracing` directly.

The main flexibility that Hermes SDK provides is that, by making the core logic use `CanLog`, this makes it possible for other external projects to choose a different logging library altogether, instead of forcing everyone who uses Hermes SDK to use Tracing.