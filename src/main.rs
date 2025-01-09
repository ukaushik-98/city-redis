use anyhow::Result;
use clap::Parser;
use tokio::{io::AsyncWriteExt, net::TcpListener};

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
            match stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").await {
                Ok(_) => println!("Pusehd 200"),
                Err(err) => panic!("{}", err),
            }
        });
    }
}
