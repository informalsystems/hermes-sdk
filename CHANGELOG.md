# Changelog

## v0.2.0 (pre-release)

-  Protobuf encoding improvements [#432](https://github.com/informalsystems/hermes-sdk/pull/432)
    - Redesign `impl_type_url!` macro to implement `SchemaGetter` on existing component type.
    - Schema components that only use `impl_type_url!` directly are no longer wrapped with `DelegateEncoding`.
    - Rename `EncodeWithContext` to `WithContext`.
    - Rename `WrappedTendermintClientState` to `WasmTendermintClientState`.
    - Implement Protobuf encoding for `WasmClientMessage`.
    - Implement `MutEncoder` for `Timestamp` and `CommitmentRoot`.
    - Relax `MutEncoder` constraint for `EncodeU64ProtoField` to use `TryInto`.

-  Introduce cosmos- and wasm- encoding components crates [#431](https://github.com/informalsystems/hermes-sdk/pull/431)
    - Add new `hermes-cosmos-encoding-components` crate.
    - Add new `hermes-wasm-encoding-components` crate.
    - Move `Height` encoding implementation to `hermes-cosmos-encoding-components`.
    - Move `WasmClientState`, `WasmConsensusState`, and related encoding implementations to `hermes-wasm-encoding-components`.

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