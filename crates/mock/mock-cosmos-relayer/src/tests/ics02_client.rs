use basecoin::store::impls::InMemoryStore;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_relayer_components::chain::traits::send_message::CanSendMessages;
use hermes_relayer_components::relay::traits::target::DestinationTarget;
use hermes_relayer_components::relay::traits::update_client_message_builder::CanBuildTargetUpdateClientMessage;
use hermes_runtime_components::traits::sleep::CanSleep;
use ibc::clients::tendermint::TENDERMINT_CLIENT_TYPE;
use ibc::core::client::context::client_state::ClientStateCommon;
use ibc::core::client::context::ClientValidationContext;
use ibc::core::host::types::identifiers::ClientId;

use crate::contexts::basecoin::MockBasecoin;
use crate::contexts::chain::MockCosmosContext;
use crate::tests::init::binary_setup;
use crate::types::error::Error;

#[tokio::test]
async fn test_create_client() -> Result<(), Error> {
    let relayer = binary_setup().await;

    let msg_create_client =
        <MockCosmosContext<MockBasecoin<InMemoryStore>> as CanBuildCreateClientPayload<
            MockCosmosContext<MockBasecoin<InMemoryStore>>,
        >>::build_create_client_payload(relayer.src_chain(), &())
        .await?;

    relayer
        .dst_chain()
        .send_messages(vec![msg_create_client])
        .await?;

    assert!(relayer
        .dst_chain()
        .ibc_context()
        .client_state(&ClientId::new(TENDERMINT_CLIENT_TYPE, 0).expect("never fails"))
        .is_ok());

    Ok(())
}

#[tokio::test]
async fn test_update_client() -> Result<(), Error> {
    let relayer = binary_setup().await;

    let msg_create_client =
        <MockCosmosContext<MockBasecoin<InMemoryStore>> as CanBuildCreateClientPayload<
            MockCosmosContext<MockBasecoin<InMemoryStore>>,
        >>::build_create_client_payload(relayer.src_chain(), &())
        .await?;

    relayer
        .dst_chain()
        .send_messages(vec![msg_create_client])
        .await?;

    relayer
        .runtime()
        .sleep(tokio::time::Duration::from_millis(200))
        .await;

    let src_current_height = relayer.src_chain().query_chain_status().await?.height;

    let msg_update_client = relayer
        .build_target_update_client_messages(DestinationTarget, &src_current_height)
        .await?;

    relayer.dst_chain().send_messages(msg_update_client).await?;

    let latest_client_state =
        <MockCosmosContext<MockBasecoin<InMemoryStore>> as CanQueryClientStateWithLatestHeight<
            MockCosmosContext<MockBasecoin<InMemoryStore>>,
        >>::query_client_state_with_latest_height(
            relayer.dst_chain(),
            &ClientId::new(TENDERMINT_CLIENT_TYPE, 0).expect("never fails"),
        )
        .await?;

    assert_eq!(latest_client_state.latest_height(), src_current_height);

    Ok(())
}
