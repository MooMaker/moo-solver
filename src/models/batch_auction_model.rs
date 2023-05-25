use crate::interactions::bytes_hex;
use crate::interactions::u256_decimal::{self, DecimalU256};
use anyhow::Result;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::{BTreeMap, HashMap};
use web3::types::H160;
use web3::types::U256;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BatchAuctionModel {
    pub tokens: BTreeMap<H160, TokenInfoModel>,
    pub orders: BTreeMap<usize, OrderModel>,
    pub metadata: Option<MetadataModel>,
    pub instance_name: Option<String>,
    pub time_limit: Option<u64>,
    pub max_nr_exec_orders: Option<u64>,
    pub auction_id: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderModel {
    pub sell_token: H160,
    pub buy_token: H160,
    #[serde(with = "u256_decimal")]
    pub sell_amount: U256,
    #[serde(with = "u256_decimal")]
    pub buy_amount: U256,
    pub allow_partial_fill: bool,
    pub is_sell_order: bool,
    pub fee: FeeModel,
    pub cost: CostModel,
    pub is_liquidity_order: bool,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TokenInfoModel {
    pub decimals: Option<u8>,
    pub external_price: Option<f64>,
    pub normalize_priority: Option<u64>,
    #[serde_as(as = "Option<DecimalU256>")]
    pub internal_buffer: Option<U256>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CostModel {
    #[serde(with = "u256_decimal")]
    pub amount: U256,
    pub token: H160,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeeModel {
    #[serde(with = "u256_decimal")]
    pub amount: U256,
    pub token: H160,
}

#[serde_as]
#[derive(Clone, Deserialize, Derivative, Serialize, PartialEq, Eq)]
#[derivative(Debug)]
pub struct InteractionData {
    pub target: H160,
    pub value: U256,
    #[derivative(Debug(format_with = "debug_bytes"))]
    #[serde(with = "bytes_hex")]
    pub call_data: Vec<u8>,
    pub exec_plan: ExecutionPlan,
    #[serde(default)]

    /// The input amounts into the AMM interaction - i.e. the amount of tokens
    /// that are expected to be sent from the settlement contract into the AMM
    /// for this calldata.
    ///
    /// `GPv2Settlement -> AMM`
    pub inputs: Vec<TokenAmount>,
    /// The output amounts from the AMM interaction - i.e. the amount of tokens
    /// that are expected to be sent from the AMM into the settlement contract
    /// for this calldata.
    ///
    /// `AMM -> GPv2Settlement`
    #[serde(default)]
    pub outputs: Vec<TokenAmount>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenAmount {
    #[serde(with = "u256_decimal")]
    pub amount: U256,
    pub token: H160,
}

pub fn debug_bytes(
    bytes: impl AsRef<[u8]>,
    formatter: &mut std::fmt::Formatter,
) -> Result<(), std::fmt::Error> {
    formatter.write_fmt(format_args!("0x{}", hex::encode(bytes.as_ref())))
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct ExecutionPlan {
    #[serde(flatten)]
    pub coordinates: ExecutionPlanCoordinatesModel,
    pub internal: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct ExecutionPlanCoordinatesModel {
    pub sequence: u32,
    pub position: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct ApprovalModel {
    pub token: H160,
    pub spender: H160,
    #[serde(with = "u256_decimal")]
    pub amount: U256,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct SettledBatchAuctionModel {
    pub orders: HashMap<usize, ExecutedOrderModel>,
    #[serde(default)]
    pub amms: HashMap<usize, UpdatedAmmModel>,
    pub ref_token: Option<H160>,
    #[serde_as(as = "HashMap<_, DecimalU256>")]
    pub prices: HashMap<H160, U256>,
    #[serde(default)]
    pub approvals: Vec<ApprovalModel>,
    pub interaction_data: Vec<InteractionData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetadataModel {
    pub environment: Option<String>,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExecutedOrderModel {
    #[serde(with = "u256_decimal")]
    pub exec_sell_amount: U256,
    #[serde(with = "u256_decimal")]
    pub exec_buy_amount: U256,
    #[serde_as(as = "Option<DecimalU256>")]
    pub exec_fee_amount: Option<U256>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdatedAmmModel {
    /// We ignore additional incoming amm fields we don't need.
    pub execution: Vec<ExecutedAmmModel>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExecutedAmmModel {
    pub sell_token: H160,
    pub buy_token: H160,
    #[serde(with = "u256_decimal")]
    pub exec_sell_amount: U256,
    #[serde(with = "u256_decimal")]
    pub exec_buy_amount: U256,
    pub exec_plan: Option<ExecutionPlanCoordinatesModel>,
}
