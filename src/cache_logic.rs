use serde_json::json;
#[path = "./fetch_logic.rs"]
mod fetch_logic;
use redis::aio::Connection;
use redis::{AsyncCommands, Commands};
use solana_client::rpc_client::RpcClient;
use std::sync::Arc;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn cache(rpc_client: Arc<RpcClient>, mut con: Connection) -> serde_json::Value {
    let counts = fetch_logic::fetch(rpc_client.clone()).await;
    let (all_unique_tokens, all_streams, active_streams, total_value_sent, total_value_locked) =
        counts;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let json_res: serde_json::Value = json!({
        "data": [{"target":"unique_tokens", "datapoints": [all_unique_tokens, now]},
        {"target":"streams_created", "datapoints": [all_streams, now]},
        {"target":"active_streams", "datapoints": [active_streams, now]},
        {"target":"value_sent", "datapoints": [total_value_sent, now]},
        {"target":"value_locked", "datapoints": [total_value_locked, now]}],
        "last_fetch": now
    });
    let _: () = con.set("data", &json_res.to_string()).await.unwrap();
    return json_res;
}
