use alloc::sync::Arc;

use cosmos_client_components::traits::message::CosmosMessage;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::lock::Mutex;
use tendermint::abci::Event as AbciEvent;
use tokio::sync::oneshot::Sender as SenderOnce;

use crate::types::error::Error;

pub type CosmosBatchPayload = (
    Vec<Arc<dyn CosmosMessage>>,
    SenderOnce<Result<Vec<Vec<Arc<AbciEvent>>>, Error>>,
);

pub type CosmosBatchSender = Arc<Mutex<UnboundedSender<CosmosBatchPayload>>>;

pub type CosmosBatchReceiver = UnboundedReceiver<CosmosBatchPayload>;
