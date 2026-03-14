use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 9090));
    let listener = TcpListener::bind(&addr).await.unwrap();
    
    println!("OTP Crypto Service listening on: {}", addr);
    
    loop {
        let (_socket, _) = listener.accept().await.unwrap();
        // Handle connections
    }
}
