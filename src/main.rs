use second2go::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> // Bubble up the io::Error if we failed to bind the address
{
    // Bubble up the io::Error if we failed to bind the address
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    run(listener)?.await
}
