use tokio::net::TcpSocket;
use tokio::io::AsyncReadExt;
use std::str;


pub async fn get_banner(host: &String) -> Option<String> {
    let socket = TcpSocket::new_v4().expect("Could not create another TCP socket");
    let socket_addr = host.to_owned() + ":22";
    let addr = socket_addr.parse().ok()?;
    let mut stream = socket.connect(addr).await.ok()?;
    let readbuf: &mut [u8] = &mut [0; 4096];
    let readsize: usize = stream.read(readbuf).await.ok()?;
    return Some(str::from_utf8(&readbuf[..readsize]).ok()?.to_owned());
}
