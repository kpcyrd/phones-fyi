use crate::errors::*;
use std::path::Path;
use tokio::fs;
use tokio::io::{self, AsyncBufRead, AsyncRead, BufReader};
use tokio_stream::StreamExt;
use tokio_util::io::StreamReader;

pub async fn fetch(url: &str, file: Option<&Path>) -> Result<String> {
    let text = if let Some(file) = file {
        fs::read_to_string(file).await?
    } else {
        reqwest::get(url).await?.text().await?
    };
    Ok(text)
}

pub enum FetchStream {
    File(BufReader<tokio::fs::File>),
    Network(Box<dyn AsyncBufRead + Unpin>),
}

impl FetchStream {
    pub async fn create(url: &str, file: Option<&Path>) -> Result<Self> {
        if let Some(file) = file {
            let f = tokio::fs::File::open(file).await?;
            let f = BufReader::new(f);
            Ok(FetchStream::File(f))
        } else {
            let resp = reqwest::get(url).await?;
            let reader = StreamReader::new(
                resp.bytes_stream()
                    .map(|result| result.map_err(io::Error::other)),
            );
            Ok(FetchStream::Network(Box::new(reader)))
        }
    }
}

impl AsyncRead for FetchStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match &mut *self {
            FetchStream::File(f) => std::pin::Pin::new(f).poll_read(cx, buf),
            FetchStream::Network(n) => std::pin::Pin::new(n).poll_read(cx, buf),
        }
    }
}

impl AsyncBufRead for FetchStream {
    fn poll_fill_buf(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<&[u8]>> {
        match self.get_mut() {
            FetchStream::File(f) => std::pin::Pin::new(f).poll_fill_buf(cx),
            FetchStream::Network(n) => std::pin::Pin::new(n).poll_fill_buf(cx),
        }
    }

    fn consume(self: std::pin::Pin<&mut Self>, amt: usize) {
        match self.get_mut() {
            FetchStream::File(f) => std::pin::Pin::new(f).consume(amt),
            FetchStream::Network(n) => std::pin::Pin::new(n).consume(amt),
        }
    }
}
