use std::collections::HashMap;
use serde_json::json;
#[path = "./fetch_logic.rs"]
mod fetch_logic;
use redis::aio::Connection;
use redis::{AsyncCommands, Commands};
use solana_client::rpc_client::RpcClient;
use std::sync::Arc;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::fetch_logic::TokenInfo;

pub async fn cache(rpc_client: Arc<RpcClient>, mut con: Connection) -> serde_json::Value {
    let resp = fetch_logic::fetch(rpc_client.clone()).await;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let json_res: serde_json::Value = json!({
        "data": [{"target":"unique_tokens", "datapoints": [resp.no_tokens, now]},
        {"target":"streams_created", "datapoints": [resp.no_streams, now]},
        {"target":"active_streams", "datapoints": [resp.no_active_streams, now]},
        {"target":"value_sent", "datapoints": [resp.total_value_sent, now]},
        {"target":"value_locked", "datapoints": [resp.total_value_locked, now]}],
        "last_fetch": now,
        "token_data": resp.tokens
    });
    let _: () = con.set("data", &json_res.to_string()).await.unwrap();
    return json_res;
}
