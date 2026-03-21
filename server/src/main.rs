use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:30492").await?;
    println!("Server listening on 0.0.0.0:30492");

    let mut buf = [0u8; 1024];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("Received {} bytes from {}", len, addr);

        // echo
        socket.send_to(&buf[..len], &addr).await?;
    }
}
