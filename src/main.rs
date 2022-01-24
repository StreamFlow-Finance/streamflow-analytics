use serde::Deserialize;
use serde_json::{from_slice, json};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct TokenStreamData {
    magic: u128,
    created_at: u128,
    withdrawn_amount: u128,
    canceled_at: u128,
    closable_at: u128,
    last_withdrawn_at: u128,
    sender: Pubkey,
    sender_tokens: Pubkey,
    recipient: Pubkey,
    recipient_tokens: Pubkey,
    mint: Pubkey,
    escrow_tokens: Pubkey,
    start_time: u128,
    end_time: u128,
    deposited_amount: u128,
    total_amount: u128,
    period: u128,
    cliff: u128,
    cliff_amount: u128,
    cancelable_by_sender: bool,
    cancelable_by_recipient: bool,
    withdrawal_public: bool,
    transferable_by_sender: bool,
    transferable_by_recipient: bool,
    release_rate: u128,
    stream_name: String,
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
    let owner = Pubkey::from_str("8aqyH9t4hSbyv2J3HB6t2c1AuR9dGpC5Nfk9F3jn7FjQ").unwrap();
    let usdc_token = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
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
