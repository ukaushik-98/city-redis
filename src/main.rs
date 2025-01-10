use anyhow::Result;
use clap::Parser;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // number of replicas
    #[arg(short, long, default_value_t = 2)]
    replica_count: u8,

    // master port
    #[arg(short, long, default_value_t = 8999)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let address = format!("127.0.0.1:{}", args.port);
    let listener = TcpListener::bind(&address).await?;
    println!("Listening on {}", address);
    loop {
        let (mut stream, _) = listener.accept().await?;
        let _ = tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            // In a loop, read data from the stream and write the data back.
            loop {
                let n = stream
                    .read_buf(&mut buf)
                    .await
                    .expect("failed to read data from stream");

                if n == 0 {
                    return;
                }

                stream
                    .write_all(b"HTTP/1.1 200 OK\r\n\r\n")
                    .await
                    .expect("failed to write data to stream");
            }
        });
    }
}
