use std::net::TcpListener;
use sentry_webhook::{
    startup::{run},
    config::get_config,
};

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let config = get_config().expect("Cannot read configuration");
    let socket_addr = format!("127.0.0.1:{}", config.application_port);
    let addr = socket_addr.clone();
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(socket_addr).expect("Cannot bind to socket");
    
    run(listener).await
}
