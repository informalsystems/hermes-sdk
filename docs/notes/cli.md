# CLI Components

This document provides some high level overview on how to develop CLI components for Hermes SDK.

Hermes SDK offers a modular way of building CLI applications. This is done by separating the implementations into multiple layers.

## Application Context

The application context, typically named `App`, is the main context for the CLI. The `App` context indirectly contains all other contexts that are used by the relayer, such as `Chain`, `Relay`, `BiRelay`, `Build`, and `Bootstrap`. Since it is a composite of all other contexts, it may require some advanced understanding on how to compose a hierarchy of abstract CGP contexts together.

Aside from containing other contexts, the `App` context also implements the CLI traits that are used to implement the actual CLI.

## Command Runner

The main trait that is used to implement the CLI is `CanRunCommand`/`CommandRunner`:

```rust
#[cgp_component {
    provider: CommandRunner,
    context: App,
}]
#[async_trait]
pub trait CanRunCommand<Args>: HasOutputType + HasAsyncErrorType
where
    Args: Async,
{
    async fn run_command(&self, args: &Args) -> Result<Self::Output, Self::Error>;
}
```

The `CanRunCommand` trait is generic over an `Args` type, which is used to contain the CLI arguments that are parsed from the main function. Typically, we make use of CLI libraries such as `clap` to implement the parsing of the `Args` type, and then pass it to `run_command` once the argument is constructed.

The trait is parameterized by generic `Args` type, because usually there are multiple types of command line arguments. CLI libraries like `clap` allows grouping of the arguments using subcommands, but the execution of each subcommand still needs to be implemented individually. Hence, each implementation of the subcommand can be identified through different `Args` type, which is then linked together to form the final CLI application.

The `run_command` method also returns an abstract `Output` type, which can be used to represent the output produced by the CLI. Currently, the concrete `Output` type is a legacy of the CLI implementation in Hermes v1.

There could also be other approaches of producing the output, such as by printing directly to STDOUT or STDERR. In such cases the abstract `Output` type could be instantiated to just `()`.

Similar to other parts of Hermes SDK, `CommandRunner` is implemented for `UseDelegate`, so that the main implementation of `CommandRunner` can be dispatched based on the `Args` type.

## Arg Parser

The `Args` type contains the CLI arguments that have been parsed using libraries like `clap`. However, typically such parsing is done at the low level, such as parsing certain arguments as strings or integers. On the other hand, a `CommandRunner` implementation may expect high level domain types, such as `ClientId`, to be extracted from `Args`.

One way to handle this is by implementing the parsing logic using the library-specific method, such as with the `Parser` trait in `clap`. However, doing so also means that we need to implement additional logic that is tied to specific CLI library.

In Hermes SDK, we instead define the `CanParseArg`/`ArgParser` trait to enable library-agnostic parsing of CLI arguments, which can also potentially be context-generic:

```rust
#[cgp_component {
    provider: ArgParser,
    context: App,
}]
pub trait CanParseArg<Args, Tag>: HasAsyncErrorType {
    type Parsed: Async;

    fn parse_arg(&self, args: &Args, tag: PhantomData<Tag>) -> Result<Self::Parsed, Self::Error>;
}
```

The `CanParseArg` trait is parameterized by an `Args` type, which is expected to be the same `Args` that is handled by a `CommandRunner` provider. Additionally, it also accepts a phantom `Tag` type, which is used to differentiate which field it should read from the `Args`. The tag type is required, because we may want to parse from multiple fields of the same type, such as `client_id_a` and `client_id_b`.

The `CanParseArg` trait is very similar to the `HasField` trait, but with several main differences. First, it could return an error, in case when it failed to parse from the raw values. Other than that, it also accepts a `&self` argument from the `App` context, which can be used for example to retrieve default values from the `App` context.

In fact, `CanParseArg` is a superset of `HasField`, and a trivial implementation of it can be to make use of `Args: HasField<Tag>` to simply retrieve the raw value from `Args` without further parsing.

Hermes SDK provides several context-generic implementations of `ArgParser`. For example, `ParseFromOptionalString` allows parsing of a value from a raw string, from an `Option<String>` field in `Args`.

There is also a `UseDelegate` implementation for `ArgParser`, so that the main implementation of `ArgParser` in the `App` context can be dispatched based on _both_ the `Args` type and the `Tag` type.

## Custom Arg Parser

In most cases, the implementation of arg parsing is a matter of wiring the parsing of arg fields with the correct context-generic provider. However, in some cases the logic for parsing arguments could be complicated and context-specific.

For example, the parsing and construction of of `InitChannelOptions` is actually consist of parsing of multiple fields from `Args`, and then constructing the composite `InitChannelOptions`. In such cases, the parser would necessarily needs to be context-specific.

It is also worth noting that the `Tag` in `ArgParser` can be "virtual", i.e. with the actual field not being present. For instance, `HermesApp` implements `CanParseArg<CreateChannelArgs, symbol!("init_channel_options")>` using `ParseInitCosmosChannelOptions`. But instead of reading from an `init_channel_options` field, `ParseInitCosmosChannelOptions` reads from the `target_connection_id`, `version`, and `ordering` fields to construct `CosmosInitChannelOptions`. Hence, the `Tag` type should be treated as informative only, with no assumption of the actual field being present in `Args`.

## Putting Everything Together

In summary, the Hermes SDK CLI is mainly consist of 3 modular parts: an abstract `App` context that combines other contexts, a collection of context-generic and context-specific `CommandRunner` providers that are dispatched using `UseDelegate`, and a collection of context-generic and context-specific `ArgParser` providers that are dispatched using `UseDelegate`.

By separating `ArgParser` from `CommandRunner`, it makes it much more easier to implement context-generic `CommandRunner` providers. Furthermore, the actual access of `Args` can be generalized by making use of `clap` to parse the raw arguments, and using `HasField` to enable access from a context-generic `ArgParser`.

## Custom Args

Hermes SDK currently relies on external libraries such as `clap` to parse the raw arguments. However, most of the provider implementation of `ArgParser` and `CommandRunner` are generic over any `Args`. This makes it easy for specific CLI applications to customize and define their own `Args` types. As long as the `Args` type contains the expected field, we can reuse the CLI components even when we define new `Args` types in new CLI applications.

This is especially useful, because the concrete `Args` type is typically overloaded to also define the UX for the CLI interface. This includes metadata such as the CLI argument names, whether it is required, and the help text. By decoupling the parsing from the `Args`, we make it possible to define custom `Args` type separately from the provider crate.

Nevertheless, it is common for `CommandRunner` providers to also provide a default `Args` struct that can be used together with that command runner. For example, `RunStartRelayerCommand` implements `CommandRunner<App, Args>` for any `Args`, but also exports a default `StartRelayerArgs` in the same module. The user is then free to use `StartRelayerArgs` together with `RunStartRelayerCommand` without needing to define additional `Args` type.

However, a CLI application is also free to wire `RunStartRelayerCommand` with a different `Args` type. For example, in ibc-starknet, we wire `RunStartRelayerCommand` with a custom `Args` that _renames_ the CLI arguments from `--client-id-a` and `--client-id-b` to `--starknet-client-id` and `--cosmos-client-id`. And because the provider is already generic over any `Args`, we don't need to re-implement the entire start command just because we use a custom start `Args`.

## CLI Framework

Aside from the CGP components, Hermes SDK also currently has a legacy `hermes-cli-framework` crate that was inherit from Hermes v1. Most of the implementation there are considered deprecated. When possible, all CLI implementations should migate to be implemented directly in `hermes-cli-components`, and the implementations should be made context-generic.
