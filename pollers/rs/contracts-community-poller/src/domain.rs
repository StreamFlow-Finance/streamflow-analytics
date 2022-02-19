use std::ascii::escape_default;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use serde::{Serialize, Deserialize};


/// The struct containing instructions for initializing a stream
#[repr(C)]
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct StreamInstruction {
    pub start_time: u64,
    pub end_time: u64,
    pub deposited_amount: u64,
    pub total_amount: u64,
    pub period: u64,
    pub cliff: u64,
    pub cliff_amount: u64,
    /// Whether or not a stream can be canceled by a sender (currently not used, set to TRUE)
    pub is_cancelable_by_sender: bool,
    /// Whether or not a stream can be canceled by a recipient (currently not used, set to FALSE)
    pub is_cancelable_by_recipient: bool,
    /// Whether or not a 3rd party can initiate withdraw in the name of recipient (currently not used, set to FALSE)
    pub is_withdrawal_public: bool,
    /// Whether or not a recipient can transfer the stream (currently not used, set to TRUE)
    pub is_transferable: bool,
    //4 bytes of padding to make the struct size multiple of 64 bits (8 bytes), non-meaningful data.
    pub padding: u32,
}

impl Default for StreamInstruction {
    fn default() -> Self {
        StreamInstruction {
            start_time: 0,
            end_time: 0,
            deposited_amount: 0,
            total_amount: 0,
            period: 1,
            cliff: 0,
            cliff_amount: 0,
            is_cancelable_by_sender: true,
            is_cancelable_by_recipient: false,
            is_withdrawal_public: false,
            is_transferable: true,
            padding: 0,
        }
    }
}

impl StreamInstruction {
    pub fn new(
        start_time: u64,
        end_time: u64,
        total_amount: u64,
        period: u64,
        cliff: u64,
        cliff_amount: u64,
    ) -> Self {
        Self {
            start_time,
            end_time,
            total_amount,
            deposited_amount: total_amount,
            period,
            cliff,
            cliff_amount,
            is_cancelable_by_sender: true,
            is_cancelable_by_recipient: false,
            is_withdrawal_public: false,
            is_transferable: true,
            padding: 0,
        }
    }
}

/// TokenStreamData is the struct containing metadata for an SPL token stream.
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct TokenStreamData {
    /// Magic bytes, will be used for version of the contract
    pub magic: u64,
    /// Timestamp when stream was created
    pub created_at: u64,
    /// Amount of funds withdrawn
    pub withdrawn_amount: u64,
    /// Timestamp when stream was canceled (if canceled)
    pub canceled_at: u64,
    /// Timestamp at which stream can be safely canceled by a 3rd party
    /// (Stream is either fully vested or there isn't enough capital to
    /// keep it active)
    pub cancellable_at: u64,
    /// Timestamp of the last withdrawal
    pub last_withdrawn_at: u64,
    /// Pubkey of the stream initializer
    pub sender: Pubkey,
    /// Pubkey of the stream initializer's token account
    pub sender_tokens: Pubkey,
    /// Pubkey of the stream recipient
    pub recipient: Pubkey,
    /// Pubkey of the stream recipient's token account
    pub recipient_tokens: Pubkey,
    /// Pubkey of the token mint
    pub mint: Pubkey,
    /// Pubkey of the account holding the locked tokens
    pub escrow_tokens: Pubkey,
    /// The stream instruction
    pub ix: StreamInstruction,
}

/// TokenStreamData is the struct containing metadata in human readable format
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct ReadableStreamData {
    /// Magic bytes, will be used for version of the contract
    pub magic: u64,
    /// Timestamp when stream was created
    pub created_at: u64,
    /// Amount of funds withdrawn
    pub withdrawn_amount: u64,
    /// Timestamp when stream was canceled (if canceled)
    pub canceled_at: u64,
    /// Timestamp at which stream can be safely canceled by a 3rd party
    /// (Stream is either fully vested or there isn't enough capital to
    /// keep it active)
    pub cancellable_at: u64,
    /// Timestamp of the last withdrawal
    pub last_withdrawn_at: u64,
    /// Pubkey of the stream initializer
    pub sender: String,
    /// Pubkey of the stream initializer's token account
    pub sender_tokens: String,
    /// Pubkey of the stream recipient
    pub recipient: String,
    /// Pubkey of the stream recipient's token account
    pub recipient_tokens: String,
    /// Pubkey of the token mint
    pub mint: String,
    /// Pubkey of the account holding the locked tokens
    pub escrow_tokens: String,
    /// The stream instruction
    pub ix: StreamInstruction,
}

impl ReadableStreamData {

    pub fn new(data: TokenStreamData) -> ReadableStreamData {
        ReadableStreamData{
            magic: data.magic,
            created_at: data.created_at,
            withdrawn_amount: data.withdrawn_amount,
            canceled_at: data.canceled_at,
            cancellable_at: data.cancellable_at,
            last_withdrawn_at: data.last_withdrawn_at,
            sender: data.sender.to_string(),
            sender_tokens: data.sender_tokens.to_string(),
            recipient: data.recipient.to_string(),
            recipient_tokens: data.recipient_tokens.to_string(),
            mint: data.mint.to_string(),
            escrow_tokens: data.escrow_tokens.to_string(),
            ix: data.ix
        }
    }

}