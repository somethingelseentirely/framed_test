use bytes::Bytes;
use futures::future;
use futures::stream::{SplitSink, SplitStream, TryStreamExt};
use futures_util::{SinkExt, StreamExt};
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tokio::io::Stdin;
use tokio::io::{self, AsyncReadExt};
use tokio::prelude::*;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() {
    eprintln!("Opening log for reading.");
    let input = io::stdin();

    let mut input_stream = FramedRead::new(input, BytesCodec::new());

    while let Some(data) = input_stream.next().await {
        eprintln!("Got: {}", data);
    }
}
