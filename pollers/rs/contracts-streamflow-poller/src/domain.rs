use std::ascii::escape_default;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use serde::{Serialize, Deserialize};

/// The struct containing instructions for initializing a stream
#[repr(C)]
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct StreamInstruction {
    /// Timestamp when the tokens start vesting
    pub start_time: u64,
    /// Deposited amount of tokens
    pub net_amount_deposited: u64,
    /// Time step (period) in seconds per which the vesting/release occurs
    pub period: u64,
    /// Amount released per period. Combined with `period`, we get a release rate.
    pub amount_per_period: u64,
    /// Vesting contract "cliff" timestamp
    pub cliff: u64,
    /// Amount unlocked at the "cliff" timestamp
    pub cliff_amount: u64,
    /// Whether or not a stream can be canceled by a sender
    pub cancelable_by_sender: bool,
    /// Whether or not a stream can be canceled by a recipient
    pub cancelable_by_recipient: bool,
    /// Whether or not a 3rd party can initiate withdraw in the name of recipient
    pub automatic_withdrawal: bool,
    /// Whether or not the sender can transfer the stream
    pub transferable_by_sender: bool,
    /// Whether or not the recipient can transfer the stream
    pub transferable_by_recipient: bool,
    /// Whether topup is enabled
    pub can_topup: bool,
    /// The name of this stream
    pub stream_name: [u8; 64],
    /// Withdraw frequency
    pub withdraw_frequency: u64,
    /// used as padding len in serialization in old streams, added for backwards compatibility
    pub ghost: u32,
    /// Whether or not the contract can be paused
    pub pausable: bool,
    /// Whether or not the contract can update release amount
    pub can_update_rate: bool,
}


/// TokenStreamData is the struct containing metadata for an SPL token stream.
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[repr(C)]
pub struct TokenStreamData {
    /// Magic bytes
    pub magic: u64,
    /// Version of the program
    pub version: u8,
    /// Timestamp when stream was created
    pub created_at: u64,
    /// Amount of funds withdrawn
    pub amount_withdrawn: u64,
    /// Timestamp when stream was canceled (if canceled)
    pub canceled_at: u64,
    /// Timestamp at which stream can be safely canceled by a 3rd party
    /// (Stream is either fully vested or there isn't enough capital to
    /// keep it active)
    pub end_time: u64,
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
    /// Escrow account holding the locked tokens for recipient
    pub escrow_tokens: Pubkey,
    /// Streamflow treasury authority
    pub streamflow_treasury: Pubkey,
    /// Escrow account holding the locked tokens for Streamflow (fee account)
    pub streamflow_treasury_tokens: Pubkey,
    /// The total fee amount for streamflow
    pub streamflow_fee_total: u64,
    /// The withdrawn fee amount for streamflow
    pub streamflow_fee_withdrawn: u64,
    /// Fee percentage for Streamflow
    pub streamflow_fee_percent: f32,
    /// Streamflow partner authority
    pub partner: Pubkey,
    /// Escrow account holding the locked tokens for Streamflow partner (fee account)
    pub partner_tokens: Pubkey,
    /// The total fee amount for the partner
    pub partner_fee_total: u64,
    /// The withdrawn fee amount for the partner
    pub partner_fee_withdrawn: u64,
    /// Fee percentage for partner
    pub partner_fee_percent: f32,
    /// The stream instruction
    pub ix: StreamInstruction,
    /// Padding for `ix: CreateParams` to allow for future upgrades.
    pub ix_padding: Vec<u8>,
    // Stream is closed
    pub closed: bool,
    /// time of the current pause. 0 signifies unpaused state
    pub current_pause_start: u64,
    /// total time the contract was paused for
    pub pause_cumulative: u64,
    /// timestamp of last rate change for this stream.
    /// Rate can be changed with `update` instruction
    pub last_rate_change_time: u64,
    /// Accumulated unlocked tokens before last rate change (excluding cliff_amount)
    pub funds_unlocked_at_last_rate_change: u64,
}

/// The struct containing instructions for initializing a stream
#[repr(C)]
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct ReadableStreamInstruction {
    /// Timestamp when the tokens start vesting
    pub start_time: u64,
    /// Deposited amount of tokens
    pub net_amount_deposited: u64,
    /// Time step (period) in seconds per which the vesting/release occurs
    pub period: u64,
    /// Amount released per period. Combined with `period`, we get a release rate.
    pub amount_per_period: u64,
    /// Vesting contract "cliff" timestamp
    pub cliff: u64,
    /// Amount unlocked at the "cliff" timestamp
    pub cliff_amount: u64,
    /// Whether or not a stream can be canceled by a sender
    pub cancelable_by_sender: bool,
    /// Whether or not a stream can be canceled by a recipient
    pub cancelable_by_recipient: bool,
    /// Whether or not a 3rd party can initiate withdraw in the name of recipient
    pub automatic_withdrawal: bool,
    /// Whether or not the sender can transfer the stream
    pub transferable_by_sender: bool,
    /// Whether or not the recipient can transfer the stream
    pub transferable_by_recipient: bool,
    /// Whether topup is enabled
    pub can_topup: bool,
    /// Withdraw frequency
    pub withdraw_frequency: u64,
    /// Whether or not the contract can be paused
    pub pausable: bool,
    /// Whether or not the contract can update release amount
    pub can_update_rate: bool,
}

impl ReadableStreamInstruction{

    pub fn new(data: StreamInstruction) -> ReadableStreamInstruction{
        ReadableStreamInstruction{
            start_time: data.start_time,
            net_amount_deposited: data.net_amount_deposited,
            period: data.period,
            amount_per_period: data.amount_per_period,
            cliff: data.cliff,
            cliff_amount: data.cliff_amount,
            cancelable_by_sender: data.cancelable_by_sender,
            cancelable_by_recipient: data.cancelable_by_recipient,
            automatic_withdrawal: data.automatic_withdrawal,
            transferable_by_sender: data.transferable_by_sender,
            transferable_by_recipient: data.transferable_by_recipient,
            can_topup: data.can_topup,
            withdraw_frequency: data.withdraw_frequency,
            pausable: data.pausable,
            can_update_rate: data.can_update_rate
        }
    }
}

/// TokenStreamData is the struct containing metadata in human readable format
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct ReadableStreamData {
    /// Magic bytes
    pub magic: u64,
    /// Version of the program
    pub version: u8,
    /// Timestamp when stream was created
    pub created_at: u64,
    /// Amount of funds withdrawn
    pub amount_withdrawn: u64,
    /// Timestamp when stream was canceled (if canceled)
    pub canceled_at: u64,
    /// Timestamp at which stream can be safely canceled by a 3rd party
    /// (Stream is either fully vested or there isn't enough capital to
    /// keep it active)
    pub end_time: u64,
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
    /// Escrow account holding the locked tokens for recipient
    pub escrow_tokens: String,
    /// Streamflow treasury authority
    pub streamflow_treasury: String,
    /// Escrow account holding the locked tokens for Streamflow (fee account)
    pub streamflow_treasury_tokens: String,
    /// The total fee amount for streamflow
    pub streamflow_fee_total: u64,
    /// The withdrawn fee amount for streamflow
    pub streamflow_fee_withdrawn: u64,
    /// Fee percentage for Streamflow
    pub streamflow_fee_percent: f32,
    /// Streamflow partner authority
    pub partner: String,
    /// Escrow account holding the locked tokens for Streamflow partner (fee account)
    pub partner_tokens: String,
    /// The total fee amount for the partner
    pub partner_fee_total: u64,
    /// The withdrawn fee amount for the partner
    pub partner_fee_withdrawn: u64,
    /// Fee percentage for partner
    pub partner_fee_percent: f32,
    /// The stream instruction
    pub ix: ReadableStreamInstruction,
    // Stream is closed
    pub closed: bool,
    pub multisig: u64,
}

impl ReadableStreamData {

    pub fn new(data: TokenStreamData) -> ReadableStreamData {
        ReadableStreamData{
            magic: data.magic,
            version: data.version,
            created_at: data.created_at,
            amount_withdrawn: data.amount_withdrawn,
            canceled_at: data.canceled_at,
            end_time: data.end_time,
            last_withdrawn_at: data.last_withdrawn_at,
            sender: data.sender.to_string(),
            sender_tokens: data.sender_tokens.to_string(),
            recipient: data.recipient.to_string(),
            recipient_tokens: data.recipient_tokens.to_string(),
            mint: data.mint.to_string(),
            escrow_tokens: data.escrow_tokens.to_string(),
            streamflow_treasury: data.streamflow_treasury.to_string(),
            streamflow_treasury_tokens: data.streamflow_treasury_tokens.to_string(),
            streamflow_fee_total: data.streamflow_fee_total,
            streamflow_fee_withdrawn: data.streamflow_fee_withdrawn,
            streamflow_fee_percent: data.streamflow_fee_percent,
            partner: data.partner.to_string(),
            partner_tokens: data.partner_tokens.to_string(),
            partner_fee_total: data.partner_fee_total,
            partner_fee_withdrawn: data.partner_fee_withdrawn,
            partner_fee_percent: data.partner_fee_percent,
            ix: ReadableStreamInstruction::new(data.ix),
            closed: data.closed,
            multisig: 0
        }
    }

}