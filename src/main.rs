mod fetch_logic;
use actix_web::web::Data;
use actix_web::{
    error, get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use redis::{AsyncCommands, Client, Commands, FromRedisValue, Value};
use serde_json::json;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio;
use tokio::runtime::Runtime;

#[derive(Clone)]
struct AppClients {
    rpc_client: Arc<RpcClient>,
    redis_client: Client,
}

#[get("/query")]
async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("hit /query");
    match req.app_data::<AppClients>() {
        Some(d) => {
            let mut con = d.redis_client.get_async_connection().await.unwrap();
            let res: Value = con.get("data").await.unwrap();
            let is_fetched = match res {
                Value::Nil => false,
                _ => true,
            };
            if is_fetched {
                println!("Redis data found");
                // println!("{:#?}", res);
                let val: String = FromRedisValue::from_redis_value(&res).unwrap();
                // println!("{:#?}", val);
                let form: serde_json::Value = serde_json::from_str(&val).unwrap();
                // println!("{:#?}", form);
                Ok(HttpResponse::Ok().json(form))
            } else {
                let counts = fetch_logic::fetch(d.rpc_client.clone()).await;
                let (
                    all_unique_tokens,
                    all_streams,
                    active_streams,
                    total_value_sent,
                    total_value_locked,
                ) = counts;
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                // hack
                let string = format!("{}", now);
                let val = serde_json::value::Value::from_str(&string).unwrap();
                // -
                let json_res = json!([{"target":"unique_tokens", "datapoints": [all_unique_tokens, val]},
                {"target":"streams_created", "datapoints": [all_streams, val]},
                {"target":"active_streams", "datapoints": [active_streams, val]},
                {"target":"value_sent", "datapoints": [total_value_sent, val]},
                {"target":"value_locked", "datapoints": [total_value_locked, val]}]);
                let _: () = con.set("data", &json_res.to_string()).await.unwrap();
                Ok(HttpResponse::Ok().json(json_res))
            }
        }
        _ => Err(error::ErrorBadRequest("overflow")),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Server Started!");
    let timeout = Duration::from_secs(60);
    let url = "https://api.mainnet-beta.solana.com".to_string();
    let rpc_client = Arc::new(RpcClient::new_with_timeout(url, timeout));
    let redis_client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let clients = AppClients {
        rpc_client: rpc_client.clone(),
        redis_client: redis_client.clone(),
    };
    println!("Redis and RPC Clients connected!");
    HttpServer::new(move || App::new().app_data(clients.clone()).service(index))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}
