mod fetch_logic;
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    fetch_logic::fetch().await;
    format!("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
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
//     fetch_logic::fetch().await;
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
