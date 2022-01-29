mod fetch_logic;
use actix_web::web::Data;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
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
async fn index(req: HttpRequest) -> impl Responder {
    // println!("rre {:#?}", req);
    match req.app_data::<Arc<RpcClient>>() {
        Some(d) => {
            // let mut rt = Runtime::new().unwrap();
            // println!("rre {:#?}", d);
            let s = d.clone();
            // tokio::spawn(async {
            // });
            // rt.block_on(async move {
            //     // println!("hello from the async block");
            //     // async_function("task0").await;
            //     // //bonus, you could spawn tasks too
            //     // tokio::spawn(async { async_function("task1").await });
            //     // tokio::spawn(async { async_function("task2").await });
            // });
            let child = thread::spawn(move || {
                let streamflow_addr =
                    Pubkey::from_str("8e72pYCDaxu3GqMfeQ5r8wFgoZSYk6oua1Qo9XpsZjX").unwrap();
                println!("rre {:#?}", streamflow_addr);
                let accounts_strmflw = s.get_program_accounts(&streamflow_addr).unwrap();
                println!("rre {:#?}", accounts_strmflw);
                // println!("Hello, world!1");
                // println!("Hello, world!2");
                accounts_strmflw
            });
            let res = child.join();
            fetch_logic::fetch(res.unwrap(), d.clone()).await;
            // println!("res {:#?}", res.unwrap());
            println!("res");
            // let refr = d.clone();
            // fetch_logic::fetch(d.clone()).await;
            format!("yo")
        }
        _ => format!("err"),
    }
    // println!("data {}", data);
    // fetch_logic::fetch(data).await;
    // format!("Hello")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let timeout = Duration::from_secs(60);
    println!("Hello, world!1.2");
    let url = "https://api.mainnet-beta.solana.com".to_string();
    let rpc_client = Arc::new(RpcClient::new_with_timeout(url, timeout));

    // println!("client {:#?}", rpc_client);
    // let app = App::new().data(rpc_client.clone()).service(index);
    HttpServer::new(move || App::new().app_data(rpc_client.clone()).service(index))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}

// mod fetch_logic;
// use actix_web::{get, web, App, HttpServer, Responder};
// use tokio;

// #[get("/")]
// async fn index() -> impl Responder {
//     tokio::spawn(async move {
//         println!("From main tokio thread");
//         fetch_logic::fetch().await;
//         // Would panic if uncommented showing no system here
//         // println!("{:?}", actix_web::rt::System::current());
//     });
//     format!("Hello")
// }

// fn main() {
//     actix_web::rt::System::with_tokio_rt(|| {
//         tokio::runtime::Builder::new_multi_thread()
//             .enable_all()
//             .worker_threads(8)
//             .thread_name("main-tokio")
//             .build()
//             .unwrap()
//     })
//     .block_on(async_main());
// }

// async fn async_main() {
//     tokio::spawn(async move {
//         println!("From main tokio thread");
//         // Would panic if uncommented showing no system here
//         // println!("{:?}", actix_web::rt::System::current());
//     });

//     HttpServer::new(|| {
//         App::new()
//             .wrap(actix_web::middleware::Logger::default())
//             .service(index)
//     })
//     .workers(8)
//     .bind("127.0.0.1:8080")
//     .expect("Couldn't bind to port 8088")
//     .run()
//     .await
//     .unwrap()
// }
