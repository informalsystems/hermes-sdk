# Hermes SDK - Next Generation IBC Relayer Framework

[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![Rust Stable][rustc-image]
![Rust 1.79+][rustc-version]

## Work in Progress

Hermes SDK is work-in-progress. At the current stage, a minimal implementation
of Cosmos-to-Cosmos relaying has been implemented, however
_this is not yet ready for production use_. We are also experimenting or
actively working to support relaying for
[Starknet](https://github.com/informalsystems/ibc-starknet) and Sovereign SDK
at the moment.

For any questions or concerns, please open an issue in this repo, or send an email to [`hermes@informal.systems`](mailto:hermes@informal.systems).

## Overview

Hermes SDK is a next-generation [IBC](https://www.ibcprotocol.dev/) relayer framework
for building high performance and modular IBC relayers. At its core, Hermes SDK makes
use of [_context-generic programming_](https://patterns.contextgeneric.dev/),
a new programming paradigm in Rust, to implement the IBC relayer as highly modular
components that can be customized and reused easily.

### Inter-Blockchain Communication (IBC) and Relaying

For readers who are new to the project, [IBC](https://www.ibcprotocol.dev/) is a protocol
that enables secure communication between two blockchains in a permissionless way.
IBC mirrors the concepts of networking protocols, with each message sent from one
chain to another being represented as an _IBC packet_.
At a high level, we can think of blockchains as _pure state machines_ that have no access
to external I/O. To facilitate communication between two blockchains, a _relayer_ is
used to deliver the IBC packets from a _source_ chain to a _destination_ chain.

We can think of IBC relaying similar to mail delivery in the real life. Consider the
case which Alice wants to send a letter to Bob. She would first put her letter in
an envelop, and write down the sender and recipient address. The letter is placed
in a mailbox at Alice's home, which is picked up by a mailman who delivers it
to Bob's home. In the case of IBC, Alice and Bob would be two chains A and B,
and the mailman would be an IBC relayer. The envelop would be an IBC packet,
and the mailboxes would be _provable storage locations_ on the respective chains.

Although the concept of IBC relaying is relatively simple, similar to real world
mail delivery, complexity arises when there are many packets need to be delivered.
There are many cross-cutting concerns in IBC relaying, including latency
(time for a packet to be delivered), throughput (number of packets delivered per timeframe),
reliability (failure recovery), efficiency (avoid delivering the same packet multiple times),
and cost (batch delivery to reduce transaction cost). Instead of choosing a specific
strategy, Hermes SDK allows different relaying strategies to be implemented to
balance different trade offs.

### Context-Generic Programming (CGP)

Hermes SDK makes heavy use of a new programming paradigm, context-generic programming,
which is developed by us to implement the relayer as a collection of loosely-coupled
components. At a high level, CGP allows code to be written to be generic over a
context type, i.e. the type that is used as `Self`. With that, we can easily define
multiple context types that re-use the same context-generic code through minimal wiring.

For readers with object-oriented programming (OOP) background, CGP shares some
similarities with advanced OOP concepts, such as mixins and dependency injection.
The main difference is that CGP offers polymorphism at _compile time_, with no
dynamic dispatch involved at runtime. With the use of associated types, CGP also
allows strongly-typed relations to be established between different types, thus
achieving type safety without any use of upcast or downcast operation.

In Hermes SDK, we make use of the [`cgp`](https://github.com/contextgeneric/cgp)
crate to define context-generic components. To learn more about CGP, readers are
encouraged to read the in-progress book,
[Context-Generic Programming Patterns](https://patterns.contextgeneric.dev/).

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

## Requirements

The crates in this project require Rust with version `1.79.0` or later.

## Hermes Guide

We have a comprehensive guide at [hermes.informal.systems](http://hermes.informal.systems).

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

## Versioning

We follow [Semantic Versioning](https://semver.org/), though APIs are still
under active development.

## Resources

- [IBC Website](https://cosmos.network/ibc)
- [IBC Specification](https://github.com/cosmos/ibc)
- [IBC Modules in Go](https://github.com/cosmos/ibc-go)
- [IBC Relayer in Typescript](https://github.com/confio/ts-relayer)
- [IBC Relayer in Go](https://github.com/cosmos/relayer)

## License

Copyright © 2023 Informal Systems Inc. and Hermes authors.

Licensed under the Apache License, Version 2.0 (the "License"); you may not use the files in this repository except in compliance with the License. You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

[build-image]: https://github.com/informalsystems/hermes-sdk/workflows/Rust/badge.svg
[build-link]: https://github.com/informalsystems/hermes-sdk/actions?query=workflow%3ARust
[license-image]: https://img.shields.io/badge/license-Apache_2.0-blue.svg
[license-link]: https://github.com/informalsystems/hermes/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-stable-blue.svg
[rustc-version]: https://img.shields.io/badge/rustc-1.79+-blue.svg