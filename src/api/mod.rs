mod solutions_result_notification;
mod solve;

use serde::de::DeserializeOwned;
use std::convert::Infallible;
use warp::{hyper::StatusCode, Filter, Rejection, Reply};
use web3::transports::Http;
use web3::Web3;

pub fn handle_all_routes(
    web3: Web3<Http>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let solve = solve::get_solve(web3);
    let notifications = solutions_result_notification::notifications_filter();
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS", "PUT", "PATCH"])
        .allow_headers(vec!["Origin", "Content-Type", "X-Auth-Token", "X-AppId"]);
    solve.or(notifications).recover(handle_rejection).with(cors)
}

// We turn Rejection into Reply to workaround warp not setting CORS headers on rejections.
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    Ok(warp::reply::with_status(
        format!("{:?}", err),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

const MAX_JSON_BODY_PAYLOAD: u64 = 1024 * 16 * 100000;

fn extract_payload<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Clone {
    // (rejecting huge payloads)...
    warp::body::content_length_limit(MAX_JSON_BODY_PAYLOAD).and(warp::body::json())
}
