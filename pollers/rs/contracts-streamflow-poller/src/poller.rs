use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
    str::FromStr,
    collections::HashMap,
    env
};

use redis::Commands;

use serde_json;
use solana_program::borsh as solana_borsh;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

use crate::domain::{TokenStreamData, ReadableStreamData};

pub fn poll() {
    let rpc_pool_token = env::var("RPC_POOL_TOKEN").unwrap();
    let solana_url = format!("https://streamflow.rpcpool.com/{}", rpc_pool_token.as_str()).to_string();
    let url = solana_url.clone();
    let rpc_client = RpcClient::new_with_timeout(url, Duration::from_secs(60));
    let redis_client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let streamflow_addr =
        Pubkey::from_str("strmRqUCoQUgGUan5YhzUZa6KqdzwX5L6FpUxfmKg5m").unwrap();

    loop {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut serialized_contracts: HashMap<String, ReadableStreamData> = HashMap::new();
        let res = rpc_client.get_program_accounts(&streamflow_addr).unwrap();
        for contract in res {
            let account_data_decoded: TokenStreamData =
                solana_borsh::try_from_slice_unchecked(&contract.1.data).unwrap();
            let mut data = ReadableStreamData::new(account_data_decoded.clone());

            let resp = rpc_client.get_account(&account_data_decoded.sender.clone());
            match resp {
                Ok(e) => {
                    if e.owner == Pubkey::from_str("SMPLecH534NA9acpos4G6x7uf3LWbCAwZQE9e8ZekMu").unwrap() {
                        data.multisig = 1
                    }
                }
                Err(_e) => {}
            }
            // if the sender is owned by squads program, add label 1

            serialized_contracts.insert(contract.0.to_string(), data);
        };

        let mut conn = redis_client.get_connection().unwrap();

        let json_data:serde_json::Value = serde_json::json!({"data": serialized_contracts, "meta": {"fetched_at": now}});
        let _: () = conn.set("contracts-streamflow", json_data.to_string()).unwrap();
        thread::sleep(Duration::from_millis(5000));
    }
}