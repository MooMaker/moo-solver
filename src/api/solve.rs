use crate::api::extract_payload;
use crate::models::batch_auction_model::{BatchAuctionModel, SettledBatchAuctionModel};
use crate::solve;
use anyhow::Result;
use serde::Serialize;
use std::convert::Infallible;
use warp::{
    hyper::StatusCode,
    reply::{self, json, with_status, Json, WithStatus},
    Filter, Rejection, Reply,
};
use web3::transports::Http;
use web3::Web3;

pub fn get_solve_request() -> impl Filter<Extract = (BatchAuctionModel,), Error = Rejection> + Clone
{
    warp::path!("solve")
        .and(warp::post())
        .and(extract_payload())
}

pub fn get_solve_response(result: Result<SettledBatchAuctionModel>) -> WithStatus<Json> {
    match result {
        Ok(solve) => reply::with_status(reply::json(&solve), StatusCode::OK),
        Err(err) => convert_get_solve_error_to_reply(err),
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Error<'a> {
    error_type: &'a str,
    description: &'a str,
}

pub fn internal_error(err: anyhow::Error) -> Json {
    json(&Error {
        error_type: "InternalServerError",
        description: &format!("{:?}", err),
    })
}

pub fn convert_get_solve_error_to_reply(err: anyhow::Error) -> WithStatus<Json> {
    tracing::error!(?err, "get_solve error");
    with_status(internal_error(err), StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn get_solve(
    web3: Web3<Http>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    get_solve_request().and_then(move |model| {
        let web3 = web3.clone();
        async move {
            let result = solve::solve(model, web3).await;
            Result::<_, Infallible>::Ok(get_solve_response(result))
        }
    })
}
