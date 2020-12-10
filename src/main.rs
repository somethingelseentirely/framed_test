use futures_util::StreamExt;
use tokio::fs::OpenOptions;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() {
    //let input = io::stdin();

    eprintln!("Opening log for reading.");
    let input = OpenOptions::new()
        .write(false)
        .read(true)
        .open("test_fifo")
        .await
        .unwrap();

    let mut input_stream = FramedRead::new(input, BytesCodec::new());
    loop {
        while let Some(read) = input_stream.next().await {
            match read {
                Ok(data) => {
                    eprintln!("Data: {}", data.len());
                }
                Err(e) => {
                    eprintln!("Err: {}", e);
                }
            }
        }
    }
}
