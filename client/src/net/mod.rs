use std::sync::Arc;

use tokio::{net::UdpSocket, sync::mpsc};

use crate::net::packet::Packet;

pub mod packet;

pub type PacketSender = mpsc::UnboundedSender<Packet>;
pub type PacketReceiver = mpsc::UnboundedReceiver<Packet>;

pub async fn start_udp_sender(
    mut rx: PacketReceiver,
    socket: Arc<UdpSocket>,
) -> anyhow::Result<()> {
    while let Some(packet) = rx.recv().await {
        socket.send(&postcard::to_allocvec(&packet)?).await?;
    }

    Ok(())
}

pub async fn start_udp_receiver(tx: PacketSender, socket: Arc<UdpSocket>) -> anyhow::Result<()> {
    let mut buf = vec![0u8; 4096];

    loop {
        let len = socket.recv(&mut buf).await?;

        if let Ok(packet) = postcard::from_bytes::<Packet>(&buf[..len]) {
            let _ = tx.send(packet);
        }
    }
}
