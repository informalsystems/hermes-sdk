use borsh::BorshSerialize;

use crate::sovereign::types::messages::bank::BankMessage;

#[derive(BorshSerialize)]
#[allow(non_camel_case_types)]
pub enum SovereignMessage {
    Accounts,
    Bank(BankMessage),
}
