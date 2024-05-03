use axum::{routing::get, Router};
use clap::Parser;
use maud::{html, Markup};
use tokio::net::TcpListener;

#[derive(Parser)]
struct Args {
    addr: String,
}

async fn root() -> Markup {
    html! {
        h1 { "Hello world!" }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let app = Router::<()>::new().route("/", get(root));
    let listener = TcpListener::bind(args.addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
