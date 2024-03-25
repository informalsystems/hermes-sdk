use borsh::BorshSerialize;

use crate::types::messages::bank::BankMessage;
use crate::types::messages::ibc::IbcMessage;

#[derive(BorshSerialize)]
pub enum SovereignMessage {
    Accounts,
    Bank(BankMessage),
    Ibc(IbcMessage),
}
