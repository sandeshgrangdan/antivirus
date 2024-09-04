mod antivirus;
use antivirus::Antivirus;
use clap::Parser;

#[tokio::main]
async fn main() {
    let mut app = Antivirus::new(antivirus::Args::parse());

    app.scan();
    app.notify().await;
}

