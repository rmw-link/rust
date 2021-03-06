use async_std::net::{SocketAddr, TcpListener, TcpStream};
use async_tungstenite::{
  accept_async,
  tungstenite::{Error, Result},
};
use futures::prelude::*;
use log::{error, info};

async fn accept(peer: SocketAddr, stream: TcpStream) {
  if let Err(e) = handle(peer, stream).await {
    match e {
      Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
      err => error!("Error processing connection: {}", err),
    }
  }
}

async fn handle(peer: SocketAddr, stream: TcpStream) -> Result<()> {
  let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

  info!("New WebSocket connection: {}", peer);

  while let Some(msg) = ws_stream.next().await {
    let msg = msg?;
    if msg.is_text() || msg.is_binary() {
      ws_stream.send(msg).await?;
    }
  }

  Ok(())
}

pub async fn listen(addr: String) -> Result<()> {
  let socket = TcpListener::bind(&addr).await?;

  while let Ok((stream, _)) = socket.accept().await {
    let peer = stream
      .peer_addr()
      .expect("connected streams should have a peer address");

    info!("Peer address: {}", peer);

    async_std::task::spawn(accept(peer, stream));
  }

  Ok(())
}
