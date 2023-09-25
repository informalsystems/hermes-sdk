use async_trait::async_trait;
use cgp_core::traits::delegate_component::DelegateComponent;
use cgp_core::traits::has_components::HasComponents;

use crate::std_prelude::*;
use crate::transaction::traits::types::HasTxTypes;

pub struct TxResponsePollerComponent;

#[async_trait]
pub trait TxResponsePoller<TxContext>
where
    TxContext: HasTxTypes,
{
    async fn poll_tx_response(
        context: &TxContext,
        tx_hash: &TxContext::TxHash,
    ) -> Result<TxContext::TxResponse, TxContext::Error>;
}

#[async_trait]
impl<TxContext, Component> TxResponsePoller<TxContext> for Component
where
    TxContext: HasTxTypes,
    Component: DelegateComponent<TxResponsePollerComponent>,
    Component::Delegate: TxResponsePoller<TxContext>,
{
    async fn poll_tx_response(
        context: &TxContext,
        tx_hash: &TxContext::TxHash,
    ) -> Result<TxContext::TxResponse, TxContext::Error> {
        Component::Delegate::poll_tx_response(context, tx_hash).await
    }
}

#[async_trait]
pub trait CanPollTxResponse: HasTxTypes {
    async fn poll_tx_response(
        &self,
        tx_hash: &Self::TxHash,
    ) -> Result<Self::TxResponse, Self::Error>;
}

#[async_trait]
impl<TxContext> CanPollTxResponse for TxContext
where
    TxContext: HasTxTypes + HasComponents,
    TxContext::Components: TxResponsePoller<TxContext>,
{
    async fn poll_tx_response(
        &self,
        tx_hash: &Self::TxHash,
    ) -> Result<Self::TxResponse, Self::Error> {
        TxContext::Components::poll_tx_response(self, tx_hash).await
    }
}
