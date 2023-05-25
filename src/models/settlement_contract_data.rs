use contracts::ethcontract::Bytes;
use serde::Serialize;
use web3::types::{H160, U256};

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub(crate) token_in: H160,
    pub(crate) amount_in: U256,
    pub(crate) token_out: H160,
    pub(crate) amount_out: U256,
    pub(crate) valid_to: U256,
    pub(crate) maker: H160,
    pub(crate) uid: Bytes<Vec<u8>>,
}
