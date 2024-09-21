use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::{BlockId, BlockTransactionsKind};
use op_alloy_network::Optimism;
use reqwest::Url;
use tokio::runtime::Runtime;

use pevm::{
    chain::{PevmChain, PevmOptimism},
    RpcStorage,
};

pub mod common;

#[test]
fn optimism_blocks_from_rpc() {
    let rpc_url = match std::env::var("OPTIMISM_RPC_URL") {
        Ok(value) if !value.is_empty() => value.parse().unwrap(),
        _ => Url::parse("https://optimism-rpc.publicnode.com").unwrap(),
    };

    for block_number in [125637307, 125637308] {
        let runtime = Runtime::new().unwrap();
        let provider = ProviderBuilder::new()
            .network::<Optimism>()
            .on_http(rpc_url.clone());
        let block = runtime
            .block_on(
                provider.get_block(BlockId::number(block_number), BlockTransactionsKind::Full),
            )
            .unwrap()
            .unwrap();
        let chain = PevmOptimism::mainnet();
        let spec_id = chain.get_block_spec(&block.header).unwrap();
        let rpc_storage = RpcStorage::new(provider, spec_id, BlockId::number(block_number - 1));
        common::test_execute_alloy(&rpc_storage, &chain, block.clone(), false);
    }
}
