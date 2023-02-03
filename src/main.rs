use std::net::SocketAddr;

mod team_service;
mod user_service;


use jsonrpsee::{RpcModule, server::ServerBuilder};
use team_service::{TeamService, TeamServiceTraitServer};
use user_service::{UserService, UserServiceTraitServer};

pub struct RpcServerImpl;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .expect("setting default subscriber failed");

    let _server_addr = run_server().await?;

    Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
    let server = ServerBuilder::default().build("0.0.0.0:7777").await?;

    let addr = server.local_addr()?;

    let mut io = RpcModule::new(());
    io.merge(UserService{}.into_rpc())?;
    io.merge(TeamService{}.into_rpc())?;

    let handle = server.start(io)?;

    // In this example we don't care about doing shutdown so let's it run forever.
    // You may use the `ServerHandle` to shut it down or manage it yourself.
    handle.stopped().await;

    Ok(addr)
}
