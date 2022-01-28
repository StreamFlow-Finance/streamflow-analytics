use bincode::deserialize;
use borsh::{BorshDeserialize, BorshSerialize};
use reqwest;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use serde_json::{from_slice, from_str, json};
use solana_account_decoder::parse_account_data::parse_account_data;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_program::borsh as solana_borsh;
use solana_program::pubkey::Pubkey;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

/// The struct containing instructions for initializing a stream
#[repr(C)]
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
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
#[derive(BorshSerialize, BorshDeserialize, Default, Debug)]
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

#[derive(Debug, Clone)]
pub struct TokenInfo {
    name: String,
    symbol: String,
    decimals: u8,
    logo: String,
    price: f64,
    amount_sent: u64,
    amount_of_streams: u64,
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
    let rand_token = Pubkey::from_str("D3cm6WRnyBct3p7vFqyTt2CaynsGPuVQT2zW6WHSTX6q").unwrap();
    // let account = client.get_account(&owner).unwrap();
    // println!("{:#?}", &account);
    // let balances = client.get_token_account_balance(&usdc_token).unwrap();
    // let balances = client
    //     .get_token_accounts_by_owner(&owner, TokenAccountsFilter::Mint(usdc_token))
    //     .unwrap();
    let mut token_list: HashMap<String, TokenInfo> = HashMap::new();
    // p["dog"] = json!("cat");
    // p["name"] = json!("poop");
    // println!("{:#?}", &p);
    // println!("{:#?}", &balances);
    let accounts_strmflw = client.get_program_accounts(&streamflow_addr).unwrap();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut active_streams: u64 = 0;
    let all_streams = accounts_strmflw.len();
    for account in accounts_strmflw {
        println!("pub key: {:#?}", account.0);
        let account_data_decoded: TokenStreamData =
            solana_borsh::try_from_slice_unchecked(&account.1.data).unwrap();
        println!("account {:#?}", &account_data_decoded);
        let mint = &account_data_decoded.mint;
        let mut new_token: JsonValue;
        if (now > account_data_decoded.ix.start_time) & (now < account_data_decoded.ix.end_time) {
            active_streams += 1;
        }
        match token_list.get(&mint.to_string()) {
            Some(dup) => {
                token_list.insert(
                    mint.to_string(),
                    TokenInfo {
                        amount_sent: dup.amount_sent + account_data_decoded.ix.deposited_amount,
                        name: dup.name.clone(),
                        symbol: dup.symbol.clone(),
                        logo: dup.logo.clone(),
                        decimals: dup.decimals,
                        price: dup.price,
                        amount_of_streams: dup.amount_of_streams + 1,
                    },
                );
            }
            _ => {
                let req_metadata = format!(
                    "https://public-api.solscan.io/token/meta?tokenAddress={}",
                    &mint
                );
                let req_price = format!("https://public-api.solscan.io/market/token/{}", &mint);
                let token_metadata = reqwest::get(req_metadata)
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                let token_price = reqwest::get(req_price).await.unwrap().text().await.unwrap();
                let res_metadata = serde_json::from_str(&token_metadata);
                let res_price = serde_json::from_str(&token_price);
                new_token = res_metadata.unwrap();
                let new_token_price: JsonValue = res_price.unwrap();
                let token_dec = client.get_token_supply(&mint).unwrap();
                println!("price {}", new_token_price);
                println!("res {:#?}", token_dec);
                let name = match &new_token["name"].as_str() {
                    Some(name) => name,
                    _ => "null",
                };
                let symbol = match &new_token["symbol"].as_str() {
                    Some(symbol) => symbol,
                    _ => "null",
                };
                let logo = match &new_token["icon"].as_str() {
                    Some(icon) => icon,
                    _ => "null",
                };
                // println!("{:#?}", teststr);
                // println!("{:#?}", String::from(name));
                // println!("{:#?}", symbol);
                // println!("{:#?}", logo);
                let price = match new_token_price["priceUsdt"].as_f64() {
                    Some(p) => p,
                    _ => 0.0,
                };
                let decimals = token_dec.decimals;
                let amt = account_data_decoded.ix.deposited_amount;
                println!("amount {}", amt);
                let mut token_info = TokenInfo {
                    name: String::from(name),
                    symbol: String::from(symbol),
                    logo: String::from(logo),
                    decimals: decimals,
                    price: price,
                    amount_sent: amt,
                    amount_of_streams: 1,
                };
                token_list.insert(mint.to_string(), token_info);
            }
        }
    }
    let all_unique_tokens = token_list.len();
    println!("{:#?}", token_list);
    // let account_data_encoded = &accounts_strmflw[0];

    // println!("{:#?}", account_data_encoded.mint);
}
