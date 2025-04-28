use hermes_prelude::*;
use ibc_proto::google::protobuf::Any;

#[cgp_component {
  provider: TxExtensionOptionsGetter,
  context: Chain,
}]
pub trait HasTxExtensionOptions {
    fn tx_extension_options(&self) -> &Vec<Any>;
}
