mod app;
mod audio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    // log level to debug for audio data
    log::set_max_level(log::LevelFilter::Trace);

    // let socket = UdpSocket::bind("0.0.0.0:0").await?;

    // socket.connect("127.0.0.1:30492").await?;

    // let msg = b"hello udp server";
    // socket.send(msg).await?;

    // let mut buf = [0u8; 1024];
    // let len = socket.recv(&mut buf).await?;

    // println!("Received: {:?}", &buf[..len]);

    eframe::run_native(
        "voi",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app::Application::new()))),
    )
    .expect("failed to start eframe");

    Ok(())
}
