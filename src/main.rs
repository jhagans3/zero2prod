use zero2prod::run;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:0";
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}