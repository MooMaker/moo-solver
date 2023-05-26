use crate::api::extract_payload;
use crate::models::cow_auction_result::AuctionResult;
use anyhow::Result;
use std::convert::Infallible;
use warp::{
    hyper::StatusCode,
    reply::{json, with_status, Json, WithStatus},
    Filter, Rejection, Reply,
};

pub fn notifications_request_filter(
) -> impl Filter<Extract = (AuctionResult,), Error = Rejection> + Clone {
    warp::path!("notify")
        .and(warp::post())
        .and(extract_payload())
}

pub fn get_notification_response(result: ()) -> WithStatus<Json> {
    with_status(json(&result), StatusCode::OK)
}

pub fn notifications_filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    notifications_request_filter().and_then(move |model| async move {
        tracing::info!("Auction result notification {:#?}", model);
        Result::<_, Infallible>::Ok(get_notification_response(()))
    })
}
