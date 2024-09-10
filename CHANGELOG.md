# Changelog

## v0.2.0 (pre-release)

- Remove unused constraints in `HasTimestampType::Timestamp`. [(#428)](https://github.com/informalsystems/hermes-sdk/pull/428)
- Update the `header` field in `CosmosUpdateClientMessage` to use `prost_types::Any` instead of `ibc_proto::Any`. [(#428)](https://github.com/informalsystems/hermes-sdk/pull/428)
- Generalize the `CreateClientMessageBuilder` instance `BuildCosmosCreateClientMessage` to `BuildAnyCreateClientMessage`, and allows generic `CreateClientPayload` that has `client_state: Counterparty::ClientState` and `consensus_state: Counterparty::ConsensusState` fields. [(#428)](https://github.com/informalsystems/hermes-sdk/pull/428)
- Rename the `ProvideCreateClientMessageOptionsType` instance in Cosmos from `ProvideCosmosCreateClientSettings` to `ProvideNoCreateClientMessageOptionsType`, with `CreateClientMessageOptions = ()`. [(#428)](https://github.com/informalsystems/hermes-sdk/pull/428)

## v0.1.0 (2024-09-03)

- Initial release of Hermes SDK crates on crates.io.