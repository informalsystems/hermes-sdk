use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use std::collections::HashMap;

use eyre::eyre;
use futures::lock::Mutex;
use hermes_relayer_components_extra::batch::types::config::BatchConfig;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_relayer::chain::cosmos::types::config::TxConfig;
use ibc_relayer::chain::handle::{BaseChainHandle, ChainHandle};
use ibc_relayer::config::filter::PacketFilter;
use ibc_relayer::config::{ChainConfig, Config};
use ibc_relayer::keyring::{AnySigningKeyPair, Secp256k1KeyPair};
use ibc_relayer::spawn::{spawn_chain_runtime_with_config, SpawnError};
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId};
use tendermint_rpc::client::CompatMode;
use tendermint_rpc::{Client, HttpClient};
use tokio::task;

use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::types::batch::CosmosBatchSender;
use crate::types::error::{BaseError, Error};
use crate::types::telemetry::CosmosTelemetry;

pub struct CosmosBuilder {
    pub config: Config,
    pub packet_filter: PacketFilter,
    pub telemetry: CosmosTelemetry,
    pub runtime: HermesRuntime,
    pub batch_config: BatchConfig,
    pub key_map: HashMap<ChainId, Secp256k1KeyPair>,
    pub chain_cache: Arc<Mutex<BTreeMap<ChainId, CosmosChain>>>,
    pub relay_cache: Arc<Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosRelay>>>,
    pub batch_senders:
        Arc<Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosBatchSender>>>,
}

impl CosmosBuilder {
    pub fn new_with_default(runtime: HermesRuntime) -> Self {
        Self::new(
            Default::default(),
            runtime,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }

    pub fn new(
        config: Config,
        runtime: HermesRuntime,
        telemetry: CosmosTelemetry,
        packet_filter: PacketFilter,
        batch_config: BatchConfig,
        key_map: HashMap<ChainId, Secp256k1KeyPair>,
    ) -> Self {
        Self {
            config,
            packet_filter,
            telemetry,
            runtime,
            batch_config,
            key_map,
            chain_cache: Default::default(),
            relay_cache: Default::default(),
            batch_senders: Default::default(),
        }
    }

    pub async fn build_chain(&self, chain_id: &ChainId) -> Result<CosmosChain, Error> {
        let chain_config =
            self.config.find_chain(chain_id).cloned().ok_or_else(|| {
                BaseError::spawn(SpawnError::missing_chain_config(chain_id.clone()))
            })?;

        self.build_chain_with_config(chain_config, self.key_map.get(&chain_id))
            .await
    }

    pub async fn build_chain_with_config(
        &self,
        chain_config: ChainConfig,
        m_keypair: Option<&Secp256k1KeyPair>,
    ) -> Result<CosmosChain, Error> {
        let runtime = self.runtime.runtime.clone();
        let chain_id = chain_config.id.clone();

        let (handle, key, chain_config) = task::block_in_place(|| -> Result<_, Error> {
            let handle = spawn_chain_runtime_with_config::<BaseChainHandle>(chain_config, runtime)
                .map_err(BaseError::spawn)?;

            let key = get_keypair(&chain_id, &handle, m_keypair)?;

            let chain_config = handle.config().map_err(BaseError::relayer)?;

            Ok((handle, key, chain_config))
        })?;

        let event_source_mode = chain_config.event_source.clone();

        let tx_config = TxConfig::try_from(&chain_config).map_err(BaseError::relayer)?;

        let mut rpc_client =
            HttpClient::new(tx_config.rpc_address.clone()).map_err(BaseError::tendermint_rpc)?;

        let status = rpc_client.status().await.unwrap();
        let compat_mode = CompatMode::from_version(status.node_info.version).unwrap();

        rpc_client.set_compat_mode(compat_mode);

        let context = CosmosChain::new(
            handle,
            tx_config,
            rpc_client,
            compat_mode,
            key,
            event_source_mode,
            self.runtime.clone(),
            self.telemetry.clone(),
        );

        Ok(context)
    }

    pub fn build_relay(
        &self,
        src_client_id: &ClientId,
        dst_client_id: &ClientId,
        src_chain: CosmosChain,
        dst_chain: CosmosChain,
        src_batch_sender: CosmosBatchSender,
        dst_batch_sender: CosmosBatchSender,
    ) -> Result<CosmosRelay, Error> {
        let relay = CosmosRelay::new(
            self.runtime.clone(),
            src_chain,
            dst_chain,
            src_client_id.clone(),
            dst_client_id.clone(),
            self.packet_filter.clone(),
            src_batch_sender,
            dst_batch_sender,
        );

        Ok(relay)
    }
}

pub fn get_keypair(
    chain_id: &ChainId,
    handle: &BaseChainHandle,
    m_keypair: Option<&Secp256k1KeyPair>,
) -> Result<Secp256k1KeyPair, Error> {
    if let Some(keypair) = m_keypair {
        let chain_config = handle.config().map_err(BaseError::relayer)?;

        // try add the key to the chain handle, in case if it is only in the key map,
        // as for the case of integration tests.
        let _ = handle.add_key(
            chain_config.key_name,
            AnySigningKeyPair::Secp256k1(keypair.clone()),
        );

        return Ok(keypair.clone());
    }

    let keypair = handle.get_key().map_err(BaseError::relayer)?;

    let AnySigningKeyPair::Secp256k1(key) = keypair else {
        return Err(
            BaseError::generic(eyre!("no Secp256k1 key pair for chain {}", chain_id)).into(),
        );
    };

    Ok(key)
}
