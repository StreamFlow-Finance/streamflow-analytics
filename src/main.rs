mod fetch_logic;
use actix_web::web::Data;
use actix_web::{
    error, get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
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

#[get("/query")]
async fn index_test(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("hit query");
    match req.app_data::<Arc<RpcClient>>() {
        Some(d) => {
            let counts = fetch_logic::fetch(d.clone()).await;
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
            Ok(HttpResponse::Ok().json(json_res))
        }
        _ => Err(error::ErrorBadRequest("overflow")),
    }
}

#[post("/query")]
async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("hit query");
    match req.app_data::<Arc<RpcClient>>() {
        Some(d) => {
            let counts = fetch_logic::fetch(d.clone()).await;
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
            Ok(HttpResponse::Ok().json(json_res))
        }
        _ => Err(error::ErrorBadRequest("overflow")),
    }
}

#[post("/search")]
async fn search() -> impl Responder {
    println!("hit search");
    let json_res = json!([
        "unique_tokens",
        "streams_created",
        "active_streams",
        "value_sent",
        "value_locked",
    ]);
    HttpResponse::Ok().json(json_res)
}

#[get("/")]
async fn test() -> impl Responder {
    format!("working")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!0");
    let timeout = Duration::from_secs(60);
    let url = "https://api.mainnet-beta.solana.com".to_string();
    let rpc_client = Arc::new(RpcClient::new_with_timeout(url, timeout));
    println!("Hello, world!1");
    HttpServer::new(move || {
        App::new()
            .app_data(rpc_client.clone())
            .service(index)
            .service(test)
            .service(search)
            .service(index_test)
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
