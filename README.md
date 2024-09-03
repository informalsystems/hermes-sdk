# Hermes SDK - Next Generation IBC Relayer Framework

[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![Rust Stable][rustc-image]
![Rust 1.79+][rustc-version]

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
achieving type safety without any use of upcast or downcast operations.

In Hermes SDK, we make use of the [`cgp`](https://github.com/contextgeneric/cgp)
crate to define context-generic components. To learn more about CGP, readers are
encouraged to read the in-progress book,
[Context-Generic Programming Patterns](https://patterns.contextgeneric.dev/).

### Heterogeneous Chain Communication

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

To expand beyond Cosmos, the cor relaying logic of Hermes SDK is fully
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

### Type-Safe Relaying

### Build Your Own Relayer

### Generic Chain Client Libraries

### Reproducible Test Framework


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