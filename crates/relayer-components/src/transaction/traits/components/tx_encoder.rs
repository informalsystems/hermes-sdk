use async_trait::async_trait;

use crate::std_prelude::*;
use crate::transaction::traits::types::HasTxTypes;
use cgp_core::traits::delegate_component::DelegateComponent;
use cgp_core::traits::has_components::HasComponents;
use cgp_core::traits::sync::Async;

pub struct TxEncoderComponent;

#[async_trait]
pub trait TxEncoder<TxContext>: Async
where
    TxContext: HasTxTypes,
{
    async fn encode_tx(
        context: &TxContext,
        signer: &TxContext::Signer,
        nonce: &TxContext::Nonce,
        fee: &TxContext::Fee,
        messages: &[TxContext::Message],
    ) -> Result<TxContext::Transaction, TxContext::Error>;
}

#[async_trait]
pub trait CanEncodeTx: HasTxTypes {
    async fn encode_tx(
        &self,
        signer: &Self::Signer,
        nonce: &Self::Nonce,
        fee: &Self::Fee,
        messages: &[Self::Message],
    ) -> Result<Self::Transaction, Self::Error>;
}

#[async_trait]
impl<TxContext, Component> TxEncoder<TxContext> for Component
where
    TxContext: HasTxTypes,
    Component: DelegateComponent<TxEncoderComponent>,
    Component::Delegate: TxEncoder<TxContext>,
{
    async fn encode_tx(
        context: &TxContext,
        signer: &TxContext::Signer,
        nonce: &TxContext::Nonce,
        fee: &TxContext::Fee,
        messages: &[TxContext::Message],
    ) -> Result<TxContext::Transaction, TxContext::Error> {
        Component::Delegate::encode_tx(context, signer, nonce, fee, messages).await
    }
}

#[async_trait]
impl<TxContext> CanEncodeTx for TxContext
where
    TxContext: HasTxTypes + HasComponents,
    TxContext::Components: TxEncoder<TxContext>,
{
    async fn encode_tx(
        &self,
        signer: &Self::Signer,
        nonce: &Self::Nonce,
        fee: &Self::Fee,
        messages: &[Self::Message],
    ) -> Result<Self::Transaction, Self::Error> {
        TxContext::Components::encode_tx(self, signer, nonce, fee, messages).await
    }
}
