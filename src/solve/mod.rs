use crate::interactions::settlement_contract::MooSettlementInteraction;
use crate::interactions::Interaction;
use crate::models::batch_auction_model::{
    ApprovalModel, BatchAuctionModel, ExecutedOrderModel, InteractionData, OrderModel,
    SettledBatchAuctionModel, TokenAmount, TokenInfoModel,
};
use crate::models::settlement_contract_data::Order;
use anyhow::{anyhow, Result};
use contracts::ethcontract::Bytes;
use contracts::MooSettlementContract;
use hex::FromHex;
use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use web3::transports::Http;
use web3::types::{H160, U256};
use web3::Web3;

static ALREADY_EXECUTED: AtomicBool = AtomicBool::new(false);
// const MOO_SETTLEMENT_CONTRACT_ADDRESS: &str = "0xcEe38fB7D7c6ed6BABc18898BDEF67ED572Cc9D0";
const MOO_SETTLEMENT_CONTRACT_ADDRESS: &str = "0x6d64978ec6Dc0b0175897F1b3F13BB9E6396C7e3";

pub async fn solve(
    BatchAuctionModel { orders, tokens, .. }: BatchAuctionModel,
    web3: Web3<Http>,
) -> Result<SettledBatchAuctionModel> {
    let token_in = H160::from_str("0x6778ac35e1c9aca22a8d7d820577212a89544df9").unwrap();
    let token_out = H160::from_str("0xb4fbf271143f4fbf7b91a5ded31805e42b2208d6").unwrap();
    println!("check");
    if let Some((index, order_model)) = orders
        .into_iter()
        .find(|(_, order)| order.sell_token == token_in && order.buy_token == token_out)
    {
        if ALREADY_EXECUTED.swap(true, Ordering::Relaxed) {
            return Ok(Default::default());
        }
        let amount_in = U256::from_str_radix("100000000000000000", 10).unwrap();
        let amount_out = U256::from_str_radix("1000000000000000000", 10).unwrap();

        let ref_token = get_ref_token(&tokens).unwrap();
        println!("in");

        let executed_order = ExecutedOrderModel {
            exec_sell_amount: amount_in,
            exec_buy_amount: amount_out,
            exec_fee_amount: order_model.allow_partial_fill.then_some(100.into()),
        };
        let mut calculated_prices = HashMap::new();
        let decimals = tokens.get(&ref_token).unwrap().decimals.unwrap_or(18);
        let ref_token_price = U256::exp10(decimals as usize);
        calculated_prices.insert(ref_token, ref_token_price);
        calculate_prices_for_order(&order_model, amount_in, amount_out, &mut calculated_prices)?;

        let contract = MooSettlementContract::at(
            &web3,
            H160::from_str(MOO_SETTLEMENT_CONTRACT_ADDRESS).unwrap(),
        );

        let settlement_contract_order = Order {
            token_in,
            amount_in,
            token_out,
            amount_out,
            valid_to: U256::from(1747179217),
            maker: H160::from_str("d1f5c19d7330F333F28A5CF3F391Bf679aC55841").unwrap(),
            uid: Bytes([1].to_vec()),
        };

        let signature = Vec::from_hex("3bdf1180a463da09ffc18d45937e66767cb76044b85a6108751796c8ab8a01130f5c5dad1d5282a1f5880bbc6ddb8fe194e427523a5ea32ddf1d858a23113ffd1c").unwrap();
        let interaction = MooSettlementInteraction {
            order: settlement_contract_order,
            signature: signature.into(),
            moo: contract,
        }
        .encode();

        let encoded_interaction = interaction.first().unwrap();

        let approval = ApprovalModel {
            token: token_in,
            spender: encoded_interaction.target,
            amount: amount_in,
        };
        let interaction_data = InteractionData {
            target: encoded_interaction.target,
            value: encoded_interaction.value,
            call_data: encoded_interaction.call_data.0.clone(),
            exec_plan: Default::default(),
            inputs: vec![TokenAmount {
                amount: amount_in,
                token: order_model.sell_token,
            }],
            outputs: vec![TokenAmount {
                amount: amount_out,
                token: order_model.buy_token,
            }],
        };

        Ok(SettledBatchAuctionModel {
            orders: HashMap::from([(index, executed_order)]),
            amms: Default::default(),
            ref_token: Some(ref_token),
            prices: calculated_prices,
            approvals: vec![approval],
            interaction_data: vec![interaction_data],
        })
    } else {
        Ok(SettledBatchAuctionModel::default())
    }
}

fn get_ref_token(tokens: &BTreeMap<H160, TokenInfoModel>) -> Option<H160> {
    tokens
        .iter()
        .fold(None, |result: Option<(&H160, &TokenInfoModel)>, current| {
            if let Some(result) = result {
                if current.1.normalize_priority > result.1.normalize_priority {
                    Some(current)
                } else {
                    Some(result)
                }
            } else {
                Some(current)
            }
        })
        .map(|(token_address, _)| token_address)
        .cloned()
}

fn calculate_prices_for_order(
    order: &OrderModel,
    sell_amount: U256,
    buy_amount: U256,
    prices: &mut HashMap<H160, U256>,
) -> Result<()> {
    match (prices.get(&order.sell_token), prices.get(&order.buy_token)) {
        (Some(price_sell_token), None) => {
            prices.insert(
                order.buy_token,
                price_sell_token
                    .checked_mul(sell_amount)
                    .unwrap()
                    .checked_div(buy_amount)
                    .unwrap(),
            );
        }
        (None, Some(price_buy_token)) => {
            prices.insert(
                order.sell_token,
                price_buy_token
                    .checked_mul(buy_amount)
                    .unwrap()
                    .checked_div(sell_amount)
                    .unwrap(),
            );
        }
        _ => return Err(anyhow!("can't deal with such a ring")),
    }

    Ok(())
}
