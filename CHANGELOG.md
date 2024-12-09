# Changelog

## v0.2.0 (pre-release)

-  Update `cgp` crate to v0.2.0 - [#491](https://github.com/informalsystems/hermes-sdk/pull/491)
    - For a full list of changes, refer to [contextgeneric/cgp#42](https://github.com/contextgeneric/cgp/pull/42)
      and the `cgp` [changelog](https://github.com/contextgeneric/cgp/blob/v0.2.0/CHANGELOG.md#v020-2025-12-08).

-  Bootstrap components improvements - [#473](https://github.com/informalsystems/hermes-sdk/pull/473)
    - Introduce `CosmosSdkConfigModifier` trait, which Cosmos bootstrap contexts now required to implement.
    - Implement `UseContext` for various accessor traits for the bootstrap context.
    - Pass `Bootstrap::ChainGenesisConfig` to `build_chain_with_node_config` and `build_relayer_chain_config`.
    - Rename fields in Cosmos bootstrap contexts.

- CLI Components Improvements - [#472](https://github.com/informalsystems/hermes-sdk/pull/472)
    - Implement `WithProvider` for CLI type traits.
    - Implement `UseDelegate` for `ArgParser` and `CommandRunner`.
    - Make `take_chain_process` work with `&mut ChainDriver`
    - Pass `ConfigUpdater` to `RunBootstrapChainCommand` for non-owned implementation.

-  Delegate all create and update client types and methods based on counterparty - [#468](https://github.com/informalsystems/hermes-sdk/pull/468)
    - In `CosmosChainClientComponents`, delegate the following components to `UseDelegate<DelegateCosmosChainComponents>`:
    `CreateClientPayloadTypeComponent`, `UpdateClientPayloadTypeComponent`,
    `CreateClientPayloadOptionsTypeComponent`, `CreateClientPayloadBuilderComponent`,
    `UpdateClientPayloadBuilderComponent`.
        - To use CosmosChain with a concrete counterparty, the respective components need to be implemented in
          `<DelegateCosmosChainComponents as DelegateComponent<Counterparty>>::Delegate`.

- Make message senders return new `MessageResponse` type instead of `Vec<Chain::Event>` - [#460](https://github.com/informalsystems/hermes-sdk/pull/460)
    - Introduce `HasMessageResponseType` trait with `Chain::MessageResponse` abstract type,
      to represent the response returned from the blockchain after processing a `Chain::Message`.
    - Introduce `HasMessageResponseEvents` trait for extracting `Vec<Chain::Event>` from `Chain::MessageResponse`.
    - Provide `UseEventsMessageResponse` to implement `Chain::MessageResponse` as `Vec<Chain::Event>`
      to follow the original behavior.
    - Change the method `CanSendMessages::send_messages` to return `Vec<Chain::MessageResponse>` instead of
      `Vec<Vec<Chain::Event>>`.
    - Change `CanParseTxResponseAsEvents` to `CanParseTxMessageResponse`, to extract message responses from a transaction.
    - Change the following event extractors to extract from `Chain::MessageResponse` instead of `Chain::Event`:
      `try_extract_create_client_event`, `try_extract_channel_open_init_event`, `try_extract_channel_open_try_event`,
      `try_extract_connection_open_init_event`, `try_extract_connection_open_try_event`.

- Delegate `ClientState` and `ConsensusState` types based on counterparty - [#459](https://github.com/informalsystems/hermes-sdk/pull/459)
    - Implement `UseDelegate` for `HasClientStateType`, `HasConsensusStateType`, `HasClientStateFields`, and `HasConsensusStateFields`.
    - In `CosmosChainClientComponents`, delegate the following components to `UseDelegate<DelegateCosmosChainComponents>`:
      `ClientStateTypeComponent`, `ConsensusStateTypeComponent`, `ClientStateFieldsComponent`, `ConsensusStateFieldComponent`.
        - To use CosmosChain with a concrete counterparty, the respective components need to be implemented in
          `<DelegateCosmosChainComponents as DelegateComponent<Counterparty>>::Delegate`.
    - Add `PhantomData` tag parameters to `query_client_state` and `query_consensus_state` to help in type inference when used
      with concrete contexts.

- Improve error messages when submitting Wasm client proposals - [#458](https://github.com/informalsystems/hermes-sdk/pull/458)
    - Refactor `poll_proposal_status` to accept a list of wanted status.
    - Make Cosmos implementation of `query_proposal_status` return error if the proposal failed.
    - Introduce `ProposalFailed` error type that needs to be handled by `ErrorRaiser` implementation.

- Minor encoding refactoring - [#453](https://github.com/informalsystems/hermes-sdk/pull/453)
    - Replace `DelegateEncoding` with `UseDelegate` from `cgp`.
    - Remove `DecodeViaWasmClientState`.

-  Implement abstract and mock IBC v2 components - [#445](https://github.com/informalsystems/hermes-sdk/pull/445)
    - Rename `DelegateTo` to `UseDelegate`.
    - Move the following chain trait types to `hermes-chain-type-components`: `HasAddressType`, `HasAmountType`,
      `HasDenomType`, `HasCommitmentPrefixType`.

-  CGP Refactoring [#440](https://github.com/informalsystems/hermes-sdk/pull/440)
    - Update `cgp` version to include the addition of `cgp-type`. [cgp#23](https://github.com/contextgeneric/cgp/pull/23)
    - Use `DelegateTo` from `cgp-component` instead of custom constructs to implement delegated chain implementations.
    - Use `UseContext` from `cgp-component` instead of custom `WithContext` for encoding forwarding implementations.
    - Experiment on using `WithProvider` and `HasType` from `cgp-type` to simplify chain type implementations for
      `HasHeight`, `HasMessageType`, and `HasEventType`.

- Chain components refactoring [#438](https://github.com/informalsystems/hermes-sdk/pull/438)
    - Introduce `hermes-chain-type-components` and `hermes-chain-components` crates, and move abstract chain
      constructs from `hermes-relayer-components`.
    - Split out `HasIbChainTypes` to the respective type traits - `HasConnectionIdType`, `HasChannelIdType`,
      `HasPortIdType`, `HasSequenceType`.
    - Separate `HasTimestampType` into `HasTimeType` and `HasTimeoutType`.
    - Refactor `HasIbcPacketTypes` to only `HasOutgoingPacketType`, and use `Counterparty::OutgoingPacket`
      as incoming packet.
    - Remove `HasRelayChains::Packet` associated alias type.

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
    - Generalize the `CreateClientMessageBuilder` instance `BuildCosmosCreateClientMessage` to
      `BuildAnyCreateClientMessage`, and allows generic `CreateClientPayload` that has `client_state: Counterparty::ClientState`
      and `consensus_state: Counterparty::ConsensusState` fields.
    - Rename the `ProvideCreateClientMessageOptionsType` instance in Cosmos from `ProvideCosmosCreateClientSettings`
      to `ProvideNoCreateClientMessageOptionsType`, with `CreateClientMessageOptions = ()`.

## v0.1.0 (2024-09-03)

- Initial release of Hermes SDK crates on crates.io.