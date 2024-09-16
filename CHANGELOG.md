# Changelog

## v0.2.0 (pre-release)

- Implement Protobuf encoding components for direct encoding from domain types [#430](https://github.com/informalsystems/hermes-sdk/pull/430)
    - Implement generic `MutEncoder` components for `hermes-encoding-components`.
    - Implement Protobuf `MutEncoder` components for direct Protobuf encoding without requiring all types to implement `prost::Message`.
    - Implement Protobuf `MutEncoder`s for `Any`, `Height`, `WasmClientState`, and `WasmConsensusState`.
    - Remove `ProtoWasmClientState` and `ProtoWasmConsensusState` Protobuf types.
    - Reorganize encoder implementations for `hermes-encoding-components`.

- Minor Refactoring[(#428)](https://github.com/informalsystems/hermes-sdk/pull/428)
    - Remove unused constraints in `HasTimestampType::Timestamp`.
    - Update the `header` field in `CosmosUpdateClientMessage` to use `prost_types::Any` instead of `ibc_proto::Any`.
    - Generalize the `CreateClientMessageBuilder` instance `BuildCosmosCreateClientMessage` to `BuildAnyCreateClientMessage`, and allows generic `CreateClientPayload` that has `client_state: Counterparty::ClientState` and `consensus_state: Counterparty::ConsensusState` fields.
    - Rename the `ProvideCreateClientMessageOptionsType` instance in Cosmos from `ProvideCosmosCreateClientSettings` to `ProvideNoCreateClientMessageOptionsType`, with `CreateClientMessageOptions = ()`.

## v0.1.0 (2024-09-03)

- Initial release of Hermes SDK crates on crates.io.