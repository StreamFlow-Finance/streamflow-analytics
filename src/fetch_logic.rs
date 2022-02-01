use actix_web::web::Data;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
use borsh::{BorshDeserialize, BorshSerialize};
use reqwest;
use serde_json::Value as JsonValue;
use serde::{Serialize, Deserialize};
use solana_client::rpc_client::RpcClient;
use solana_program::borsh as solana_borsh;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;

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

#[derive(Debug, Clone, Serialize)]
pub struct TokenInfo {
    name: String,
    symbol: String,
    decimals: u8,
    logo: String,
    price: f64,
    amount_sent: u64,
    amount_of_streams: u64,
    value: f64,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub no_tokens: usize,
    pub no_streams: usize,
    pub no_active_streams: u64,
    pub total_value_sent: f64,
    pub total_value_locked: f64,
    pub tokens : HashMap<String, TokenInfo>
}

pub async fn fetch(client: Arc<RpcClient>) -> Response {
    let s = client.clone();
    let child1 = thread::spawn(move || {
        let streamflow_addr =
            Pubkey::from_str("8e72pYCDaxu3GqMfeQ5r8wFgoZSYk6oua1Qo9XpsZjX").unwrap();
        println!("rre {:#?}", streamflow_addr);
        let res = s.get_program_accounts(&streamflow_addr).unwrap();
        println!("rre {:#?}", res);
        res
    });
    let accounts_strmflw = child1.join().unwrap();
    let all_streams = accounts_strmflw.len();
    let mut token_list: HashMap<String, TokenInfo> = HashMap::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut active_streams: u64 = 0;
    let mut total_value_sent: f64 = 0.0;
    let mut total_value_locked: f64 = 0.0;
    for account in accounts_strmflw {
        println!("pub key: {:#?}", account.0);
        let account_data_decoded: TokenStreamData =
            solana_borsh::try_from_slice_unchecked(&account.1.data).unwrap();
        println!("account {:#?}", &account_data_decoded);
        let mint = &account_data_decoded.mint;
        let mut new_token: JsonValue;
        let mut is_active: bool = false;
        if (now > account_data_decoded.ix.start_time) & (now < account_data_decoded.ix.end_time) {
            active_streams += 1;
            is_active = true
        }
        match token_list.get(&mint.to_string()) {
            Some(dup) => {
                let divisor = u64::pow(10, dup.decimals as u32);
                let new_amt = dup.amount_sent + account_data_decoded.ix.deposited_amount;
                let new_value = (new_amt / divisor) as f64 * dup.price;
                if is_active {
                    total_value_locked += new_value - dup.value;
                }
                total_value_sent += new_value - dup.value;
                token_list.insert(
                    mint.to_string(),
                    TokenInfo {
                        amount_sent: new_amt,
                        name: dup.name.clone(),
                        symbol: dup.symbol.clone(),
                        logo: dup.logo.clone(),
                        decimals: dup.decimals,
                        price: dup.price,
                        amount_of_streams: dup.amount_of_streams + 1,
                        value: new_value,
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
                println!("https");
                let tempmint = mint.clone();
                let tempclient = client.clone();
                let child = thread::spawn(move || tempclient.get_token_supply(&tempmint).unwrap());
                let token_dec = child.join().unwrap();
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
                let divisor = u64::pow(10, decimals as u32);
                let adjusted_for_decimals = amt / divisor;
                let value = adjusted_for_decimals as f64 * price;
                println!(
                    "divisor {}\nadjusted {}\noriginal {}\nfinal {}",
                    divisor, adjusted_for_decimals, amt, value
                );
                if is_active {
                    total_value_locked += value;
                }
                total_value_sent += value;
                let mut token_info = TokenInfo {
                    name: String::from(name),
                    symbol: String::from(symbol),
                    logo: String::from(logo),
                    decimals: decimals,
                    price: price,
                    amount_sent: amt,
                    amount_of_streams: 1,
                    value,
                };
                token_list.insert(mint.to_string(), token_info);
            }
        }
    }
    let all_unique_tokens = token_list.len();
    println!("{:#?}", token_list);

    // number of different tokens being streamed (vested)
    println!("unique tokens {:#?}", all_unique_tokens);
    // total number of streams created (ever)
    println!("total streams {:#?}", all_streams);
    // total number of active streams (active)
    println!("total active streams {:#?}", active_streams);
    // total value of all vesting contracts ever created (in USD)
    println!("total value sent {:#?}", total_value_sent);
    // number of different tokens being streamed (vested)
    println!("total value locked {:#?}", total_value_locked);

    Response {
        no_tokens: all_unique_tokens,
        no_streams: all_streams,
        no_active_streams: active_streams,
        total_value_sent,
        total_value_locked,
        tokens: token_list
    }
}
