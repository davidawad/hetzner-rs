use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
    time::Duration,
};

use hetzner_rs::{HttpMethod, Secret, SyncClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    let mock = thread::spawn(move || -> std::io::Result<()> {
        let (mut stream, _) = listener.accept()?;
        stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        let mut request = [0_u8; 1024];
        let _ = stream.read(&mut request)?;
        let body = r#"{"servers":[],"meta":{"pagination":{"page":1,"per_page":50,"last_page":null,"next_page":null,"previous_page":null,"total_entries":0}}}"#;
        write!(
            stream,
            "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
            body.len()
        )?;
        Ok(())
    });

    let client =
        SyncClient::with_base_url(&format!("http://{address}/v1"), Secret::new("mock-token"))?;
    let servers = client
        .call(HttpMethod::Get, |api| {
            Box::pin(api.list_servers(None, None, None, None, None, None))
        })?
        .into_inner();
    mock.join()
        .map_err(|_| std::io::Error::other("mock server thread panicked"))??;

    println!("Servers: {}", servers.servers.len());
    println!("Source: local mock");
    Ok(())
}
