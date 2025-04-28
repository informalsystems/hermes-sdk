use hermes_prelude::*;

use crate::chain::traits::HasIbcChainTypes;
use crate::relay::traits::{
    CanRelayConnectionOpenAck, CanRelayConnectionOpenConfirm, CanRelayConnectionOpenTry,
    ConnectionOpenHandshakeRelayer, ConnectionOpenHandshakeRelayerComponent, HasRelayChains,
};

/**
   Relays a connection open handshake using a connection ID that has been
   initialized at the source chain.

   Specifically, the `ConnOpenTry`, `ConnOpenAck`, and `ConnOpenConfirm` steps of
   the handshake protocol are performed between both chains. Upon successful
   completion of the handshake protocol, a connection will have been established
   between both chains.

   This can be used for relaying of connections that are created by external
   users.
*/
pub struct RelayConnectionOpenHandshake;

#[cgp_provider(ConnectionOpenHandshakeRelayerComponent)]
impl<Relay, SrcChain, DstChain> ConnectionOpenHandshakeRelayer<Relay>
    for RelayConnectionOpenHandshake
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + CanRelayConnectionOpenTry
        + CanRelayConnectionOpenAck
        + CanRelayConnectionOpenConfirm,
    SrcChain: HasIbcChainTypes<DstChain> + HasAsyncErrorType,
    DstChain: HasIbcChainTypes<SrcChain> + HasAsyncErrorType,
{
    async fn relay_connection_open_handshake(
        relay: &Relay,
        src_connection_id: &SrcChain::ConnectionId,
    ) -> Result<DstChain::ConnectionId, Relay::Error> {
        let dst_connection_id = relay.relay_connection_open_try(src_connection_id).await?;
        relay
            .relay_connection_open_ack(src_connection_id, &dst_connection_id)
            .await?;

        relay
            .relay_connection_open_confirm(src_connection_id, &dst_connection_id)
            .await?;

        Ok(dst_connection_id)
    }
}
