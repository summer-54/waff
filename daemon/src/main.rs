mod daemon;

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").ok();
    let name = std::env::var("NAME").ok();
    let password = std::env::var("PASSWORD").ok();
    if let Err(err) = daemon::start(token, name, password).await {
        log::error!("{}", err);
    }
}
