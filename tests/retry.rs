use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
    time::{Duration, Instant},
};

use hetzner_rs::{HttpMethod, RetryPolicy, Secret, SyncClient};

fn mock(statuses: Vec<u16>) -> (String, thread::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let base = format!("http://{}/v1", listener.local_addr().unwrap());
    let handle = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_millis(300);
        let mut methods = Vec::new();
        while Instant::now() < deadline && methods.len() < statuses.len() {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream
                        .set_read_timeout(Some(Duration::from_secs(2)))
                        .unwrap();
                    let mut request = [0_u8; 4096];
                    let count = stream.read(&mut request).unwrap();
                    let first_line = String::from_utf8_lossy(&request[..count]);
                    methods.push(first_line.lines().next().unwrap().to_string());
                    let status = statuses[methods.len() - 1];
                    let (reason, body) = if status == 429 && first_line.starts_with("POST ") {
                        (
                            "Too Many Requests",
                            r#"{"error":{"code":"rate_limit_exceeded","message":"slow down"}}"#,
                        )
                    } else if first_line.starts_with("POST ") {
                        (
                            "Created",
                            r#"{"action":{"command":"poweron","error":null,"finished":"2026-01-01T00:00:00Z","id":9,"progress":100,"resources":[],"started":"2026-01-01T00:00:00Z","status":"success"}}"#,
                        )
                    } else {
                        (
                            "OK",
                            r#"{"servers":[],"meta":{"pagination":{"page":1,"per_page":50,"last_page":null,"next_page":null,"previous_page":null,"total_entries":0}}}"#,
                        )
                    };
                    write!(
                        stream,
                        "HTTP/1.1 {status} {reason}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                        body.len()
                    )
                    .unwrap();
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(5));
                }
                Err(error) => panic!("mock accept failed: {error}"),
            }
        }
        methods
    });
    (base, handle)
}

fn client(base: &str) -> SyncClient {
    SyncClient::with_base_url(base, Secret::new("test-token"))
        .unwrap()
        .with_retry_policy(RetryPolicy {
            max_attempts: 4,
            initial_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
        })
}

#[test]
fn post_429_is_returned_after_exactly_one_request() {
    let (base, mock) = mock(vec![429, 200]);
    let sync = client(&base);
    let error = sync
        .call(HttpMethod::Post, |api| {
            Box::pin(api.poweron_server(std::num::NonZeroU64::new(73).unwrap()))
        })
        .unwrap_err();

    assert_eq!(error.status(), Some(reqwest::StatusCode::TOO_MANY_REQUESTS));
    assert_eq!(
        mock.join().unwrap(),
        ["POST /v1/servers/73/actions/poweron HTTP/1.1"]
    );
}

#[test]
fn get_429_is_retried_once_and_then_succeeds() {
    let (base, mock) = mock(vec![429, 200]);
    let sync = client(&base);
    let response = sync
        .call(HttpMethod::Get, |api| {
            Box::pin(api.list_servers(None, None, None, None, None, None))
        })
        .unwrap()
        .into_inner();

    assert!(response.servers.is_empty());
    assert_eq!(
        mock.join().unwrap(),
        ["GET /v1/servers HTTP/1.1", "GET /v1/servers HTTP/1.1"]
    );
}
