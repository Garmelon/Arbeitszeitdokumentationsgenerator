mod endpoints;
mod render;

use axum::{routing::get, Router};
use clap::Parser;
use tokio::net::TcpListener;

#[derive(Parser)]
struct Args {
    addr: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let app =
        Router::<()>::new().route("/", get(endpoints::index::get).post(endpoints::index::post));
    let listener = TcpListener::bind(args.addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
