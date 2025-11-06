use cityrade_common::{net::event::Event, world::World};
use tokio::net::{TcpSocket, TcpStream};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:7897";
    let stream = TcpStream::connect(addr).await?;
    info!("Connected to {}", stream.peer_addr()?);

    stream.writable().await?;
    stream.try_write(
        Event::AuthLogin {
            login: "alo".into(),
            password: "alo".into(),
        }
        .to_string()
        .as_bytes(),
    )?;

    Ok(())
}
