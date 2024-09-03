use cgp::prelude::*;
use ibc_proto::google::protobuf::Any;

#[derive_component(TxExtensionOptionsGetterComponent, TxExtensionOptionsGetter<Chain>)]
pub trait HasTxExtensionOptions {
    fn tx_extension_options(&self) -> &Vec<Any>;
}
