mod handlers;
mod pincer;
mod server;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    server::run("127.0.0.1:8080").await
}
