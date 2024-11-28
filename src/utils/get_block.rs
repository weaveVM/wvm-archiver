use {
    crate::utils::schema::Network,
    ethers_core::types::{Block, H256, U64},
    ethers_providers::{Middleware, ProviderError},
};

pub async fn by_number(number: u64) -> Result<Option<Block<H256>>, ProviderError> {
    let network: Network = Network::config();
    let provider = Network::provider(&network, false).await;
    let block = provider.get_block(number).await;

    match block {
        Ok(block) => Ok(block),
        Err(e) => Err(e),
    }
}

pub async fn get_current_block_number() -> U64 {
    let network: Network = Network::config();
    // connect to the target EVM provider
    let provider = Network::provider(&network, false).await;
    let block_number = provider.get_block_number().await.unwrap_or(0.into());
    block_number
}
