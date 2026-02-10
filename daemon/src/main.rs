mod daemon;

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").ok().map(|s| s.into());
    let name = std::env::var("NAME").ok().map(|s| s.into());
    let password = std::env::var("PASSWORD").ok().map(|s| s.into());
    if let Err(err) = daemon::start(token, name, password).await {
        log::error!("{}", err);
    }
}
