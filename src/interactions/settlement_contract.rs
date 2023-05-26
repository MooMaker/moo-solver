use crate::interactions::{EncodedInteraction, Interaction};
use {
    crate::models::settlement_contract_data::Order,
    // contracts::IZeroEx,
    contracts::MooSettlementContract,
    web3::types::Bytes,
};

#[derive(Clone, Debug)]
pub struct MooSettlementInteraction {
    pub order: Order,
    pub signature: Bytes,
    pub moo: MooSettlementContract,
}

impl Interaction for MooSettlementInteraction {
    fn encode(&self) -> Vec<EncodedInteraction> {
        let method = self.moo.swap(
            (
                self.order.token_in,
                self.order.amount_in,
                self.order.token_out,
                self.order.amount_out,
                self.order.valid_to,
                self.order.maker,
                self.order.uid.clone(),
            ),
            ethcontract::Bytes(self.signature.clone().0),
        );
        let call_data = method.tx.data.expect("no call data").0;
        vec![EncodedInteraction {
            target: self.moo.address(),
            value: 0.into(),
            call_data: ethcontract::Bytes(call_data),
        }]
    }
}
