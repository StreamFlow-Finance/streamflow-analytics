mod fetch_logic;

#[tokio::main]
async fn main() {
    fetch_logic::fetch().await
}
