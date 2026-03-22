use std::sync::Arc;

use tokio::{net::UdpSocket, sync::mpsc};

use crate::net::{start_udp_receiver, start_udp_sender};

mod app;
mod audio;
mod net;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);
    socket.connect("127.0.0.1:30492").await?;

    let (tx_out, rx_out) = mpsc::unbounded_channel();
    let (tx_in, rx_in) = mpsc::unbounded_channel();

    let s1 = socket.clone();
    tokio::spawn(async move {
        let _ = start_udp_sender(rx_out, s1).await;
    });

    let s2 = socket.clone();
    tokio::spawn(async move {
        let _ = start_udp_receiver(tx_in, s2).await;
    });

    eframe::run_native(
        "voi",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app::Application::new(tx_out, rx_in)))),
    )
    .expect("failed to start eframe");

    Ok(())
}
