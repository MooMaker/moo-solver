use crate::interactions::{bytes_hex, debug_bytes::debug_bytes, u256_decimal};
use derivative::Derivative;
use serde::Deserialize;
use std::collections::BTreeSet;
use web3::types::{AccessList, H160, U256};

/// The result a given solver achieved in the auction
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AuctionResult {
    /// Solution was valid and was ranked at the given place
    /// Rank 1 means the solver won the competition
    Ranked(usize),

    /// Solution was invalid for some reason
    Rejected(SolverRejectionReason),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SolverRejectionReason {
    /// The solver didn't return a successful response
    RunError(SolverRunError),

    /// The solution candidate didn't include any user orders
    NoUserOrders,

    /// The solution violated a price constraint (ie. max deviation to external
    /// price vector)
    PriceViolation,

    /// The solution contains custom interation/s using the token/s not
    /// contained in the allowed bufferable list Returns the list of not
    /// allowed tokens
    NonBufferableTokensUsed(BTreeSet<H160>),

    /// The solution contains non unique execution plans (duplicated
    /// coordinates)
    InvalidExecutionPlans,

    /// The solution didn't pass simulation. Includes all data needed to
    /// re-create simulation locally
    SimulationFailure(TransactionWithError),

    /// The solution doesn't have a positive score. Currently this can happen
    /// only if the objective value is negative.
    NonPositiveScore,

    /// The solution has a score that is too high. This can happen if the
    /// score is higher than the maximum score (surplus + fees)
    #[serde(rename_all = "camelCase")]
    TooHighScore {
        #[serde(with = "u256_decimal")]
        surplus: U256,
        #[serde(with = "u256_decimal")]
        fees: U256,
        #[serde(with = "u256_decimal")]
        max_score: U256,
        #[serde(with = "u256_decimal")]
        submitted_score: U256,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SolverRunError {
    Timeout,
    Solving(String),
}

/// Contains all information about a failing settlement simulation
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionWithError {
    /// Transaction data used for simulation of the settlement
    #[serde(flatten)]
    pub transaction: SimulatedTransaction,
    /// Error message from the simulator
    pub error: String,
}

/// Transaction data used for simulation of the settlement
#[derive(Clone, Deserialize, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimulatedTransaction {
    /// The simulation was done at the beginning of the block
    pub block_number: u64,
    /// Index of the transaction inside the block the transaction was simulated
    /// on
    pub tx_index: u64,
    /// Is transaction simulated with internalized interactions or without
    pub internalization: InternalizationStrategy,
    /// Which storage the settlement tries to access. Contains `None` if some
    /// error happened while estimating the access list.
    pub access_list: Option<AccessList>,
    /// Solver address
    pub from: H160,
    /// GPv2 settlement contract address
    pub to: H160,
    /// Transaction input data
    #[derivative(Debug(format_with = "debug_bytes"))]
    #[serde(with = "bytes_hex")]
    pub data: Vec<u8>,
    /// Gas price can influence the success of simulation if sender balance
    /// is not enough for paying the costs of executing the transaction onchain
    #[serde(with = "u256_decimal")]
    pub max_fee_per_gas: U256,
    #[serde(with = "u256_decimal")]
    pub max_priority_fee_per_gas: U256,
}

/// Whether or not internalizable interactions should be encoded as calldata
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum InternalizationStrategy {
    #[serde(rename = "Disabled")]
    EncodeAllInteractions,
    #[serde(rename = "Enabled")]
    SkipInternalizableInteraction,
}
