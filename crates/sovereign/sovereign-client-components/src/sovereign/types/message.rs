use borsh::BorshSerialize;

use crate::sovereign::types::messages::bank::BankMessage;

#[derive(BorshSerialize)]
pub enum SovereignMessage {
    Accounts,
    Bank(BankMessage),
}
