use std::collections::HashMap;

use ibc_relayer::chain::handle::{BaseChainHandle, ChainHandle};
use ibc_relayer::config::filter::PacketFilter;
use ibc_relayer::config::Config;
use ibc_relayer_components::build::traits::components::birelay_builder::CanBuildBiRelay;
use ibc_relayer_cosmos::contexts::birelay::CosmosBiRelay;
use ibc_relayer_cosmos::contexts::builder::CosmosBuilder;
use ibc_test_framework::error::{handle_generic_error, Error};
use ibc_test_framework::prelude::TaggedFullNodeExt;
use ibc_test_framework::types::binary::chains::ConnectedChains;

pub fn new_cosmos_builder<ChainA, ChainB>(
    config: &Config,
    chains: &ConnectedChains<ChainA, ChainB>,
    packet_filter: PacketFilter,
) -> Result<CosmosBuilder, Error>
where
    ChainA: ChainHandle,
    ChainB: ChainHandle,
{
    let runtime = chains.node_a.value().chain_driver.runtime.clone();

    let key_a = chains.node_a.wallets().value().relayer.key.clone();

    let key_b = chains.node_b.wallets().value().relayer.key.clone();

    let key_map = HashMap::from([
        (chains.chain_id_a().cloned_value(), key_a),
        (chains.chain_id_b().cloned_value(), key_b),
    ]);

    let builder = CosmosBuilder::new(
        config.clone(),
        runtime,
        Default::default(),
        packet_filter,
        Default::default(),
        key_map,
    );

    Ok(builder)
}

pub fn build_cosmos_relay_context<ChainA, ChainB>(
    config: &Config,
    chains: &ConnectedChains<ChainA, ChainB>,
    packet_filter: PacketFilter,
) -> Result<CosmosBiRelay<BaseChainHandle, BaseChainHandle>, Error>
where
    ChainA: ChainHandle,
    ChainB: ChainHandle,
{
    let runtime = chains.node_a.value().chain_driver.runtime.clone();
    let builder = new_cosmos_builder(config, chains, packet_filter)?;

    let birelay = runtime
        .block_on(builder.build_birelay(
            chains.chain_id_a().value(),
            chains.chain_id_b().value(),
            chains.client_id_a().value(),
            chains.client_id_b().value(),
        ))
        .map_err(handle_generic_error)?;

    Ok(birelay)
}
