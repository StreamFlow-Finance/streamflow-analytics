mod fetch_logic;
use actix_web::web::Data;
use actix_web::{error, get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_json::json;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio;
use tokio::runtime::Runtime;

#[get("/")]
async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
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
            let json_res = json!({
                "unique_tokens": all_unique_tokens,
                "streams_created": all_streams,
                "active_streams": active_streams,
                "value_sent": total_value_sent,
                "value_locked": total_value_locked
            });
            Ok(HttpResponse::Ok().json(json_res))
        }
        _ => Err(error::ErrorBadRequest("overflow")),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!0");
    let timeout = Duration::from_secs(60);
    let url = "https://api.mainnet-beta.solana.com".to_string();
    let rpc_client = Arc::new(RpcClient::new_with_timeout(url, timeout));
    println!("Hello, world!1");
    HttpServer::new(move || App::new().app_data(rpc_client.clone()).service(index))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}
