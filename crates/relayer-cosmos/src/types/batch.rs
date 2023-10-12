use alloc::sync::Arc;

use ibc_cosmos_client_components::traits::message::CosmosMessage;
use tendermint::abci::Event as AbciEvent;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot::Sender as SenderOnce;

use crate::types::error::Error;

pub type CosmosBatchPayload = (
    Vec<Arc<dyn CosmosMessage>>,
    SenderOnce<Result<Vec<Vec<Arc<AbciEvent>>>, Error>>,
);

pub type CosmosBatchSender = UnboundedSender<CosmosBatchPayload>;

pub type CosmosBatchReceiver = UnboundedReceiver<CosmosBatchPayload>;
