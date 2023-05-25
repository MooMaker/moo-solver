pub mod api;
mod interactions;
pub mod models;
pub mod solve;
pub mod tracing_helper;

use std::net::SocketAddr;
use tokio::{task, task::JoinHandle};
use web3::transports::Http;
use web3::Web3;

pub fn serve_task(address: SocketAddr, web3: Web3<Http>) -> JoinHandle<()> {
    let filter = api::handle_all_routes(web3);
    tracing::info!(%address, "serving api");
    task::spawn(warp::serve(filter).bind(address))
}
