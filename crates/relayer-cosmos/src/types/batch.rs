use alloc::sync::Arc;

use cosmos_client_components::traits::message::CosmosMessage;
use futures::channel::oneshot::Sender as SenderOnce;
use tendermint::abci::Event as AbciEvent;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::types::error::Error;

pub type CosmosBatchPayload = (
    Vec<Arc<dyn CosmosMessage>>,
    SenderOnce<Result<Vec<Vec<Arc<AbciEvent>>>, Error>>,
);

pub type CosmosBatchSender = UnboundedSender<CosmosBatchPayload>;

pub type CosmosBatchReceiver = UnboundedReceiver<CosmosBatchPayload>;
