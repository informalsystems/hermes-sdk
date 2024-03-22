use borsh::BorshSerialize;

use crate::sovereign::types::messages::bank::BankMessage;
use crate::sovereign::types::messages::ibc::IbcMessage;

#[derive(BorshSerialize)]
pub enum SovereignMessage {
    Accounts,
    Bank(BankMessage),
    Ibc(IbcMessage),
}
