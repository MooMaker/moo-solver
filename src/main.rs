#![recursion_limit = "256"]
use moo_solver::serve_task;
use moo_solver::tracing_helper::initialize;
use std::net::SocketAddr;
use structopt::StructOpt;
use web3::transports::Http;
use web3::Web3;

#[derive(Debug, StructOpt)]
struct Arguments {
    #[structopt(long, env = "LOG_FILTER", default_value = "warn,debug,info")]
    pub log_filter: String,
    #[structopt(long, env = "BIND_ADDRESS", default_value = "0.0.0.0:8000")]
    bind_address: SocketAddr,

    /// The relative slippage tolerance to apply to on-chain swaps.
    #[structopt(long, env, default_value = "10")]
    relative_slippage_bps: u32,

    /// The absolute slippage tolerance in native token units to cap relative
    /// slippage at. Default is 0.007 ETH.
    #[structopt(long, env)]
    absolute_slippage_in_native_token: Option<f64>,
}

#[tokio::main]
async fn main() {
    let args = Arguments::from_args();
    initialize(args.log_filter.as_str());
    tracing::info!("running data-server with {:#?}", args);

    let web3 = create_web3();
    let serve_task = serve_task(args.bind_address, web3);
    tokio::select! {
        result = serve_task => tracing::error!(?result, "serve task exited"),
    };
}

fn create_web3() -> Web3<Http> {
    let infura_key = std::env::var("INFURA_KEY").expect("Set INFURA_KEY env variable");
    let http = Http::new(format!("https://goerli.infura.io/v3/{infura_key}").as_str()).unwrap();
    Web3::new(http)
}
