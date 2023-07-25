use futures::prelude::*;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::RpcAccountInfoConfig,
};
use solana_sdk::{account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::str::FromStr;
use anchor_lang::AccountDeserialize;
use rust_decimal::prelude::*;
use rust_decimal::MathematicalOps;
use whirlpool::state::Whirlpool;

fn pricemath_sqrt_price_x64_to_price(
    sqrt_price_x64: u128,
    decimals_a: i8,
    decimals_b: i8,
) -> String {
    let sqrt_price_x64_decimal = Decimal::from_str(&sqrt_price_x64.to_string()).unwrap();

    let price = sqrt_price_x64_decimal
        .checked_div(Decimal::TWO.powu(64))
        .unwrap()
        .powu(2)
        .checked_mul(Decimal::TEN.powi((decimals_a - decimals_b) as i64))
        .unwrap();

    price.to_string()
}

#[tokio::main]
async fn main() {
    let pubkey = Pubkey::from_str("AU971DrPyhhrpRnmEBp5pDTWL2ny7nofb5vYBjDJkR2E").unwrap();
    let ws_url = String::from("wss://convincing-sly-voice.solana-mainnet.discover.quiknode.pro/5cfe6a24d39c80215daecdb6f2962762488626e2/");
    let client = PubsubClient::new(&ws_url).await.unwrap();

    let config = Some(RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64Zstd),
        commitment: Some(CommitmentConfig::confirmed()),
        data_slice: None,
        min_context_slot: None,
    });

    let (mut stream, _) = client.account_subscribe(&pubkey, config).await.unwrap();

    let w_eth_decimals = 8i8;
    let usdc_decimals = 6i8;

    loop {
        let logs = stream.next().await.unwrap();
        let account: Account = logs.value.decode().unwrap();
        let mut account_data: &[u8] = &account.data;

        let whirlpool = Whirlpool::try_deserialize(&mut account_data).unwrap();
        println!(
            "Orca whETH price: '{}' USDc",
            pricemath_sqrt_price_x64_to_price(whirlpool.sqrt_price, w_eth_decimals, usdc_decimals)
        );
    }
}
