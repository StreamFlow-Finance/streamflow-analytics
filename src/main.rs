use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let url = "https://rpc.ankr.com/solana".to_string();
    let client = RpcClient::new(url);
    // println!("{:#?}", client)
    let owner = Pubkey::from_str("8aqyH9t4hSbyv2J3HB6t2c1AuR9dGpC5Nfk9F3jn7FjQ").unwrap();
    let usdc_token = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
    let account = client.get_account(&owner).unwrap();
    println!("{:#?}", &account);
    // let balances = client.get_token_account_balance(&usdc_token).unwrap();
    let balances = client
        .get_token_accounts_by_owner(&owner, TokenAccountsFilter::Mint(usdc_token))
        .unwrap();
    println!("{:#?}", &balances);
}
