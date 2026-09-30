use std::net::SocketAddr;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::domain::scraping::request::ProxyConfig;

pub struct ProxyForwarder {
    local_addr: SocketAddr,
    _handle: tokio::task::JoinHandle<()>,
}

impl ProxyForwarder {
    pub async fn start(config: &ProxyConfig) -> Option<Self> {
        let parsed = parse_proxy_url(&config.url);
        let listener = TcpListener::bind("127.0.0.1:0").await.ok()?;
        let local_addr = listener.local_addr().ok()?;
        let upstream = parsed.upstream;
        let auth_header = parsed.auth_header;

        let handle = tokio::spawn(async move {
            loop {
                let Ok((client, _)) = listener.accept().await else {
                    break;
                };
                let up = upstream.clone();
                let auth = auth_header.clone();
                tokio::spawn(tunnel(client, up, auth));
            }
        });

        Some(Self {
            local_addr,
            _handle: handle,
        })
    }

    pub fn chrome_flag(&self) -> String {
        format!("--proxy-server=http://{}", self.local_addr)
    }
}

struct ParsedProxy {
    upstream: String,
    auth_header: Option<String>,
}

fn parse_proxy_url(url: &str) -> ParsedProxy {
    let stripped = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .unwrap_or(url);

    if let Some((userinfo, hostport)) = stripped.split_once('@') {
        let auth = STANDARD.encode(userinfo.as_bytes());
        ParsedProxy {
            upstream: hostport.to_owned(),
            auth_header: Some(format!("Basic {auth}")),
        }
    } else {
        ParsedProxy {
            upstream: stripped.to_owned(),
            auth_header: None,
        }
    }
}

async fn tunnel(mut client: TcpStream, upstream: String, auth: Option<String>) {
    let mut buf = vec![0u8; 8192];
    let n = match client.read(&mut buf).await {
        Ok(n) if n > 0 => n,
        _ => return,
    };

    let request = String::from_utf8_lossy(buf.get(..n).unwrap_or_default());
    let (first_line, rest) = request.split_once("\r\n").unwrap_or((&request, ""));

    let mut injected = String::from(first_line);
    injected.push_str("\r\n");
    if let Some(ref a) = auth {
        injected.push_str("Proxy-Authorization: ");
        injected.push_str(a);
        injected.push_str("\r\n");
    }
    injected.push_str(rest);

    let Ok(mut up) = TcpStream::connect(&upstream).await else {
        return;
    };

    if up.write_all(injected.as_bytes()).await.is_err() {
        return;
    }

    if first_line.starts_with("CONNECT ") {
        let mut reply = Vec::new();
        let mut byte = [0u8; 1];
        while matches!(up.read(&mut byte).await, Ok(1)) {
            reply.push(byte[0]);
            if reply.ends_with(b"\r\n\r\n") {
                break;
            }
        }
        if !reply.windows(4).any(|w| w == b" 200") {
            let _ = client.write_all(&reply).await;
            return;
        }
        let _ = client
            .write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n")
            .await;
    }

    let (mut cr, mut cw) = client.split();
    let (mut ur, mut uw) = up.split();
    tokio::select! {
        _ = tokio::io::copy(&mut cr, &mut uw) => {},
        _ = tokio::io::copy(&mut ur, &mut cw) => {},
    }
}
