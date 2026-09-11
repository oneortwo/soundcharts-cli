use crate::{cli::PaginationArgs, client::SoundchartsClient};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    task::JoinHandle,
};

pub fn pagination() -> PaginationArgs {
    PaginationArgs {
        limit: None,
        all: true,
        page_size: 100,
        no_paginate: false,
    }
}

pub async fn server(
    bodies: Vec<serde_json::Value>,
) -> (SoundchartsClient, JoinHandle<Vec<reqwest::Url>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let client = SoundchartsClient::for_test(&base);
    let handle = tokio::spawn(async move {
        let mut requests = Vec::new();
        for body in bodies {
            let (mut socket, _) =
                tokio::time::timeout(std::time::Duration::from_secs(5), listener.accept())
                    .await
                    .unwrap()
                    .unwrap();
            let mut request = Vec::new();
            loop {
                let mut chunk = [0; 4096];
                let n = socket.read(&mut chunk).await.unwrap();
                assert!(n > 0);
                request.extend_from_slice(&chunk[..n]);
                if request.windows(4).any(|bytes| bytes == b"\r\n\r\n") {
                    break;
                }
            }
            let request = String::from_utf8(request).unwrap();
            let target = request.split_whitespace().nth(1).unwrap();
            requests.push(reqwest::Url::parse(&format!("http://localhost{target}")).unwrap());
            let body = body.to_string();
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
            socket.write_all(response.as_bytes()).await.unwrap();
        }
        requests
    });
    (client, handle)
}
