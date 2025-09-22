mod event;

use std::{net::SocketAddr, time::Duration};

use bincode::{config, decode_from_slice};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    signal,
    task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::event::Event;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let host = "0.0.0.0:7897";

    let listener = TcpListener::bind(host).await?;
    info!("Listening on {host}");

    let cancel = CancellationToken::new();
    {
        let cancel = cancel.clone();
        tokio::spawn(async move {
            let _ = signal::ctrl_c().await;
            info!("Shutting down...");
            cancel.cancel();
        });
    }

    let mut workers = JoinSet::new();

    loop {
        tokio::select! {
            biased;

            _ = cancel.cancelled() => { break },

            res = listener.accept() => {
                let (stream, addr) = match res {
                    Ok(ok) => ok,
                    Err(err) => { error!("Error of accept connection: {}", err); continue; }
                };
                stream.set_nodelay(true)?;

                info!("[{addr}] Connected!");

                let child = cancel.child_token();
                workers.spawn(handle_connnections(stream, addr, child));
            }
        }
    }

    cancel.cancel();
    while let Some(r) = workers.join_next().await {
        debug!("...: {:#?}", r);
    }

    Ok(())
}

async fn handle_connnections(
    mut stream: TcpStream,
    addr: SocketAddr,
    cancel: CancellationToken,
) -> anyhow::Result<()> {
    let (mut stream_rx, mut stream_tx) = stream.split();
    let mut buf: Vec<u8> = vec![0u8; 65536];

    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                let _ = tokio::time::timeout(Duration::from_secs(3), stream_tx.flush()).await;
                stream_tx.shutdown().await?;
                break;
            }

            read = stream_rx.read(&mut buf) => {
                let n = match read {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(e) => {
                        error!("[{addr}] read error: {e}");
                        break;
                    }
                };

                let content = &buf[..n];
                let (decoded, _): (Event, usize) = decode_from_slice(content, config::standard())?;
                info!("[{addr}] Process message: {}", decoded);
                let result = process_message(decoded)?;

                if let Err(e) = stream_tx.write_all(&result).await {
                    error!("[{addr}] write error: {e}");
                    break;
                }
            }
        }
    }

    Ok(())
}

fn process_message(content: Event) -> anyhow::Result<Vec<u8>> {
    Ok(vec![0u8])
}
