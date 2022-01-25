use borsh::{BorshDeserialize, BorshSerialize};
use serde::Deserialize;
use serde_json::{from_slice, json};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;
use std::time::Duration;

/// The struct containing instructions for initializing a stream
#[derive(Debug, Deserialize)]
pub struct StreamInstruction {
    /// Timestamp when the tokens start vesting
    pub start_time: u64,
    /// Timestamp when all tokens are fully vested
    pub end_time: u64,
    /// Deposited amount of tokens (should be <= total_amount)
    pub deposited_amount: u64,
    /// Total amount of the tokens in the escrow account if
    /// contract is fully vested
    pub total_amount: u64,
    /// Time step (period) in seconds per which the vesting occurs
    pub period: u64,
    /// Vesting contract "cliff" timestamp
    pub cliff: u64,
    /// Amount unlocked at the "cliff" timestamp
    pub cliff_amount: u64,
    /// Whether or not a stream can be canceled by a sender
    pub cancelable_by_sender: bool,
    /// Whether or not a stream can be canceled by a recipient
    pub cancelable_by_recipient: bool,
    /// Whether or not a 3rd party can initiate withdraw in the name of recipient
    pub withdrawal_public: bool,
    /// Whether or not the sender can transfer the stream
    pub transferable_by_sender: bool,
    /// Whether or not the recipient can transfer the stream
    pub transferable_by_recipient: bool,
    /// Release rate of recurring payment
    pub release_rate: u64,
    /// The name of this stream
    pub stream_name: String,
}

// impl Default for StreamInstruction {
//     //these values are overridden.
//     fn default() -> Self {
//         StreamInstruction {
//             start_time: 0,
//             end_time: 0,
//             deposited_amount: 0,
//             total_amount: 0,
//             period: 1,
//             cliff: 0,
//             cliff_amount: 0,
//             cancelable_by_sender: true,
//             cancelable_by_recipient: false,
//             withdrawal_public: false,
//             transferable_by_sender: false,
//             transferable_by_recipient: true,
//             release_rate: 0,
//             stream_name: "Stream".to_string(),
//         }
//     }
// }

/// TokenStreamData is the struct containing metadata for an SPL token stream.
// #[repr(C)]
#[derive(Debug, Deserialize)]
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
    pub closable_at: u64,
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
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // let url = "https://rpc.ankr.com/solana".to_string();
    // let url = "https://solana-api.projectserum.com".to_string();
    let url = "https://api.mainnet-beta.solana.com".to_string();
    let timeout = Duration::from_secs(60);
    let client = RpcClient::new_with_timeout(url, timeout);
    // println!("{:#?}", client)
    // let owner = Pubkey::from_str("8aqyH9t4hSbyv2J3HB6t2c1AuR9dGpC5Nfk9F3jn7FjQ").unwrap();
    // let usdc_token = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
    let streamflow_addr = Pubkey::from_str("8e72pYCDaxu3GqMfeQ5r8wFgoZSYk6oua1Qo9XpsZjX").unwrap();
    // let account = client.get_account(&owner).unwrap();
    // println!("{:#?}", &account);
    // let balances = client.get_token_account_balance(&usdc_token).unwrap();
    // let balances = client
    //     .get_token_accounts_by_owner(&owner, TokenAccountsFilter::Mint(usdc_token))
    //     .unwrap();
    // println!("{:#?}", &balances);
    let accounts_strmflw = client.get_program_accounts(&streamflow_addr).unwrap();
    let account_data_encoded = from_slice::<TokenStreamData>(&accounts_strmflw[0].1.data);
    // let account_data_encoded = &accounts_strmflw[0].1.data;

    println!("{:#?}", account_data_encoded);
}
