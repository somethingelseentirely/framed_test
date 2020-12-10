use futures_util::StreamExt;
use std::env;
use std::process::exit;
use tokio::fs::OpenOptions;
use tokio::io::AsyncReadExt;
use tokio::time::{sleep, Duration};
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        eprintln!("You need to specify which read API to use, try one of:\n'cargo run -- framed'\n'cargo run -- async'");
        exit(1);
    }
    eprintln!("Opening log.");
    let input = OpenOptions::new()
        .write(false)
        .read(true)
        .open("test_fifo")
        .await
        .expect(
            "Couldn't open fifo. Try running the interactive explanation with './interactive.sh'",
        );
    match &args[1][..] {
        "framed" => {
            eprintln!("Reading with FramedRead.");
            let mut input_stream = FramedRead::new(input, BytesCodec::new());
            loop {
                eprintln!("Starting read loop.");
                while let Some(read) = input_stream.next().await {
                    eprintln!("Reading next frame.");
                    match read {
                        Ok(data) => {
                            eprintln!("Data: {}", data.len());
                        }
                        Err(e) => {
                            eprintln!("Err: {}", e);
                        }
                    }
                }
                eprintln!("FrameRead read 0 bytes and is in eof mode.");
                sleep(Duration::from_millis(4000)).await;
            }
        }
        "async" => {
            eprintln!("Reading with AsyncRead.");
            let mut input = input;
            loop {
                eprintln!("Reading more into buffer.");
                let mut buffer = [0; 1024];
                // The `read` method is defined by this trait.
                let read = input.read(&mut buffer[..]).await;
                match read {
                    Ok(n) => {
                        eprintln!("Data: {}", n);
                    }
                    Err(e) => {
                        eprintln!("Err: {}", e);
                    }
                }
                sleep(Duration::from_millis(4000)).await;
            }
        }
        _ => {
            eprintln!("Bad API name, try one of:\n'cargo run -- framed'\n'cargo run -- async'");
            exit(1);
        }
    }
}
