use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{
    error, get, http, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use clokwerk::{AsyncScheduler, Job, TimeUnits};
use redis::{AsyncCommands, Client, Commands, Connection, FromRedisValue, Value};
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
// Import week days and WeekDay
use clokwerk::Interval::*;

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
                let val: String = FromRedisValue::from_redis_value(&res).unwrap();
                let form: serde_json::Value = serde_json::from_str(&val).unwrap();
                Ok(HttpResponse::Ok().json(form))
            } else {
                let json_res: serde_json::Value =
                    cache_logic::cache(d.rpc_client.clone(), con).await;
                Ok(HttpResponse::Ok().json(json_res))
            }
        }
        _ => Err(error::ErrorBadRequest("overflow")),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Server Started!");
    let url = "https://api.mainnet-beta.solana.com".to_string();
    let rpc_client = Arc::new(RpcClient::new_with_timeout(url, Duration::from_secs(60)));
    let redis_client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let clients = AppClients {
        rpc_client: rpc_client.clone(),
        redis_client: redis_client.clone(),
    };
    let t_local_redis_cli = redis_client.clone();
    let t_local_rpc_cli = rpc_client.clone();
    thread::spawn(move || {
        println!("Spawned T1");
        let mut scheduler = AsyncScheduler::new();
        let t2_local_redis_cli = t_local_redis_cli.clone();
        let t2_local_rpc_cli = t_local_rpc_cli.clone();
        scheduler
            .every(10.seconds())
            .run(move || {
                let m = t2_local_redis_cli.clone();
                async move {
                    let mut con = m.get_async_connection().await.unwrap();
                    cache_logic::cache(t2_local_rpc_cli.clone(), con).await;
                }
            });
        println!("Scheduled querer.");

        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(clients.clone())
            .service(index)
            .service(schedule)
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run()
    .await
}
