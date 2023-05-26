pub(crate) mod bytes_hex;
pub mod settlement_contract;
pub(crate) mod u256_decimal;

use {
    ethcontract::Bytes,
    serde::{Deserialize, Serialize},
    std::fmt::{self, Debug, Formatter},
    web3::types::{H160, U256},
};

pub trait Interaction: std::fmt::Debug + Send + Sync {
    // TODO: not sure if this should return a result.
    // Write::write returns a result but we know we write to a vector in memory so
    // we know it will never fail. Then the question becomes whether
    // interactions should be allowed to fail encoding for other reasons.
    fn encode(&self) -> Vec<EncodedInteraction>;
}

impl Interaction for Box<dyn Interaction> {
    fn encode(&self) -> Vec<EncodedInteraction> {
        self.as_ref().encode()
    }
}

#[derive(Debug, Clone)]
pub struct EncodedInteraction {
    pub(crate) target: H160,
    pub(crate) value: U256,
    pub(crate) call_data: Bytes<Vec<u8>>,
}

// pub type EncodedInteraction = (
//     H160,           // target
//     U256,           // value
//     Bytes<Vec<u8>>, // callData
// );

impl Interaction for EncodedInteraction {
    fn encode(&self) -> Vec<EncodedInteraction> {
        vec![self.clone()]
    }
}

impl Interaction for InteractionData {
    fn encode(&self) -> Vec<EncodedInteraction> {
        vec![EncodedInteraction {
            target: self.target,
            value: self.value,
            call_data: Bytes(self.call_data.clone()),
        }]
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionData {
    pub target: H160,
    #[serde(with = "u256_decimal")]
    pub value: U256,
    #[serde(with = "bytes_hex")]
    pub call_data: Vec<u8>,
}

impl Debug for InteractionData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("InteractionData")
            .field("target", &self.target)
            .field("value", &self.value)
            .field(
                "call_data",
                &format_args!("0x{}", hex::encode(&self.call_data)),
            )
            .finish()
    }
}
