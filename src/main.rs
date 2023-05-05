use std::net::TcpListener;
use sentry_webhook::{
    startup::{run},
};

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let socket_addr = format!("127.0.0.1:{}", "9230");
    let addr = socket_addr.clone();
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(socket_addr).expect("Cannot bind to socket");
    
    run(listener).await
}
