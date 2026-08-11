use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::domain::scraping::request::ProxyConfig;

pub struct ProxyForwarder {
    local_addr: SocketAddr,
    _handle: tokio::task::JoinHandle<()>,
}

impl ProxyForwarder {
    pub async fn start(config: &ProxyConfig) -> Option<Self> {
        let parsed = parse_proxy_url(&config.url)?;
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

fn parse_proxy_url(url: &str) -> Option<ParsedProxy> {
    let stripped = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .unwrap_or(url);

    if let Some((userinfo, hostport)) = stripped.split_once('@') {
        let auth = base64_encode(userinfo.as_bytes());
        Some(ParsedProxy {
            upstream: hostport.to_owned(),
            auth_header: Some(format!("Basic {auth}")),
        })
    } else {
        Some(ParsedProxy {
            upstream: stripped.to_owned(),
            auth_header: None,
        })
    }
}

async fn tunnel(mut client: TcpStream, upstream: String, auth: Option<String>) {
    let mut buf = vec![0u8; 8192];
    let n = match client.read(&mut buf).await {
        Ok(n) if n > 0 => n,
        _ => return,
    };

    let request = String::from_utf8_lossy(&buf[..n]);
    let (first_line, rest) = request.split_once("\r\n").unwrap_or((&request, ""));

    let mut injected = String::from(first_line);
    injected.push_str("\r\n");
    if let Some(ref a) = auth {
        injected.push_str(&format!("Proxy-Authorization: {a}\r\n"));
    }
    injected.push_str(rest);

    let mut up = match TcpStream::connect(&upstream).await {
        Ok(s) => s,
        _ => return,
    };

    if up.write_all(injected.as_bytes()).await.is_err() {
        return;
    }

    if first_line.starts_with("CONNECT ") {
        let mut reply = Vec::new();
        let mut byte = [0u8; 1];
        while let Ok(1) = up.read(&mut byte).await {
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

fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in input.chunks(3) {
        let b0 = chunk[0];
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);
        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(b2 & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}
