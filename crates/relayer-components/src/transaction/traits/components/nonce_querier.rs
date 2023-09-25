use async_trait::async_trait;
use cgp_core::traits::delegate_component::DelegateComponent;
use cgp_core::traits::has_components::HasComponents;

use crate::std_prelude::*;
use crate::transaction::traits::types::HasTxTypes;

pub struct NonceQuerierComponent;

#[async_trait]
pub trait NonceQuerier<TxContext>
where
    TxContext: HasTxTypes,
{
    async fn query_nonce(
        context: &TxContext,
        signer: &TxContext::Signer,
    ) -> Result<TxContext::Nonce, TxContext::Error>;
}

#[async_trait]
pub trait CanQueryNonce: HasTxTypes {
    async fn query_nonce(&self, signer: &Self::Signer) -> Result<Self::Nonce, Self::Error>;
}

#[async_trait]
impl<TxContext, Component> NonceQuerier<TxContext> for Component
where
    TxContext: HasTxTypes,
    Component: DelegateComponent<NonceQuerierComponent>,
    Component::Delegate: NonceQuerier<TxContext>,
{
    async fn query_nonce(
        context: &TxContext,
        signer: &TxContext::Signer,
    ) -> Result<TxContext::Nonce, TxContext::Error> {
        Component::Delegate::query_nonce(context, signer).await
    }
}

#[async_trait]
impl<TxContext> CanQueryNonce for TxContext
where
    TxContext: HasTxTypes + HasComponents,
    TxContext::Components: NonceQuerier<TxContext>,
{
    async fn query_nonce(&self, signer: &Self::Signer) -> Result<Self::Nonce, Self::Error> {
        TxContext::Components::query_nonce(self, signer).await
    }
}
