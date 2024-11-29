# Problems Solved by Hermes SDK


### Heterogeneous Chain Relaying

Compared to alternative IBC relayers, Hermes SDK is built with first-class
support for IBC relaying between different implementations of blockchains.
When IBC was first developed, the initial focus was on supporting communication
between blockchains in the Cosmos ecosystem, which are implemented using
[CometBFT](https://github.com/cometbft/cometbft) and
[CosmosSDK](https://github.com/cosmos/cosmos-sdk).
As a result, many early IBC relayer implementations are strongly coupled
with Cosmos-specific assumptions on how the chain is implemented.

As IBC gains traction within Cosmos, the use of IBC is now expanding beyond
the Cosmos ecosystem, to other blockchain ecosystems such as Starknet.
A main challenge in supporting non-Cosmos blockchains with existing IBC
relayers is that these blockchains may have entirely different ways of
how to interact with the chain, including how queries are made, how
messages and events are formatted, how transactions are signed and submitted,
how to track the consensus finality of chains, etc.

To expand beyond Cosmos, the core relaying logic of Hermes SDK is fully
abstract, and makes no assumption about how to interact with a chain.
All chain types in Hermes SDK, including height, message, event,
transaction, and errors, are defined as _abstract_ associated types
that can be instantiated with any concrete chain-specific types.
Hermes SDK provides abstract interfaces such as the `MessageSender`
trait, with very little restriction on how chains can implement these
interfaces.

At the moment, we have two sub-projects that make use of Hermes SDK
to implement Cosmos to non-Cosmos relaying for
[Starknet](https://github.com/informalsystems/ibc-starknet) and
[Sovereign SDK](https://github.com/informalsystems/hermes-sovereign-relayer).

### Fully Abstract Core Relayer Logic

The core relaying logic of Hermes SDK is implemented in the
[`hermes-relayer-components`](./crates/relayer/relayer-components/) crate.
When inspecting the [`Cargo.toml` manifest](./crates/relayer/relayer-components/Cargo.toml),
you may notice that aside from the core CGP components definition, there
is no use of any implementation-specific crates, such as `std` and `tokio`.
This shows that the core logic of Hermes SDK is _fully abstract_, and can be
instantiated with any concrete implementation.

Hermes SDK not only abstract over the concrete chain implementation,
but also everything else, including error handling, logging, and async runtime.
This means that one can easily build a custom IBC relayer that runs
in restricted environment, such as on Wasm web applications, or in
sandboxed test environments such as [Kani](https://model-checking.github.io/kani/).

### Type-Safe Relaying

Hermes SDK makes heavy use of associated types to make use of types abstractly.
This allows the core relaying logic to differentiate values coming from different
chains as different abstract types, even when they have the same underlying
concrete type. Because of that, we can make use of the strong type system in
Rust to ensure that the core relaying logic is implemented correctly.

For a simplified demonstraction, consider the implementation of the mailman
delivery example that we covered earlier. After the mailman collects Alice's
letter, it is supposed read the recipient's address on the envelop, and deliver
it to Bob. However, we may make a mistake in the mailman's program, and read
the sender's address. When using common programming techniques, such an error
would only be caught during runtime, when the mailman knocks Alice's door
and try to deliver the letter she sent to Bob. But with the techniques used
in Hermes SDK, the sender's address would have a different abstract type than
the recipient's address, and hence it would result in a compilation error
in the mailman's program.

The actual logic for IBC relaying is more complicated than the simplified
mailman delivery example, so there are many more places to potentially make
such mistakes. Furthermore, it can be difficult to track where the mistake
happens, because errors may happen only after the mistaken value has flowed
to other parts of the code base.

By making use of CGP and the strong type system provided by Rust, Hermes SDK
offers unique advantage over other IBC relaying implementations that the
core relaying logic is not only correct _now_, but that it will always
remain correct in the _future_. We do not need to rely on an expert developer
to carefully audit the relayer code to ensure that it is correct. Instead,
the Rust compiler helps to the job, and will raise an error any time a
developer mistakenly mixes up values coming from different chains.

### Context-Generic Chain Clients

The primary use case for Hermes SDK is to implement IBC relaying. But to do
that, we first need to implement clients for communicating with a chain
full node. It turns out that there are sufficiently many client-side
features required, that we have implemented fairly rich chain client libraries
for Hermes SDK.

Most of the chain-specific client implementations in Hermes SDK are implemented
as context-generic components. This means that one can potentially reuse some
of the chain components to build other kinds of client-side blockchain
applications. Compared to other chain client libraries, the chain clients
provided by Hermes SDK is more extensible and customizable through CGP.

### Build Your Own Relayer

Hermes SDK provides concrete relay contexts that are ready to use, without
requiring the end user to understand CGP. However, the default relay context
may not satisfy the need for all potential use cases. For example, a chain
may have implemented some new features which require custom behavior from
the chain client.

Most commonly, users would submit feature request to the relayer's project
to add custom behavior to specific chains. However, doing so repeatedly
can bloat the relayer's code base, and make it more challenging to reason
about the special case behavior. In our case, we instead offer Hermes SDK
as a library, which users can make use of to build their own custom relayer.

With CGP, developers can reuse chain components provided by Hermes SDK,
without having to fork the entire code base. At the same time, any chain
component is fully replaceable with customized implementations.

### Reproducible Test Framework

Hermes SDK offers a comprehensive test framework for testing the relayer.
The Hermes SDK IBC test suite is written as abstract tests that can work
with any concrete chain and relay contexts.

Hermes SDK also provides chain bootstrapping implementations, which would
bootstrap new blockchain instances to be used for each test case. The
bootstrapping helps ensure that each test is reproducible, and simplifies
the need to manually setup blockchain instances before running the tests.
