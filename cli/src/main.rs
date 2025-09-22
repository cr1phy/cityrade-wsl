use std::io::IoSlice;

use bincode::{config, encode_to_vec};
use tokio::net::TcpSocket;

#[derive(bincode::Encode)]
enum Message {
    Connect,
    AuthLogin { login: String, password: Vec<u8> },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1:7897".parse()?;

    let socket = TcpSocket::new_v4()?;
    let stream = socket.connect(addr).await?;

    let bytes = encode_to_vec(
        Message::AuthLogin {
            login: String::from("name"),
            password: b"password".as_slice().to_vec(),
        },
        config::standard(),
    )?;
    println!("{}", String::from_utf8_lossy(&bytes));

    stream.writable().await?;
    stream.try_write_vectored(&[IoSlice::new(bytes.as_slice())])?;

    Ok(())
}
