use std::io::IoSlice;

use tokio::net::TcpSocket;

struct Message {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1:7897".parse()?;

    let socket = TcpSocket::new_v4()?;
    let stream = socket.connect(addr).await?;

    stream.writable().await?;
    stream.try_write_vectored(&[IoSlice::new(b"connect")])?;

    Ok(())
}
