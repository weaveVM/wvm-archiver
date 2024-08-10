use ethers_providers::{Middleware, ProviderError};
use ethers_core::types::{Block, H256};
use crate::utils::schema::Network;

pub async fn by_number(number: u64) -> Result<Option<Block<H256>>, ProviderError> {
    let network: Network = Network::config();
    let provider = Network::provider(&network, false).await;    
    let block = provider.get_block(number).await;

    match block {
        Ok(block) => Ok(block),
        Err(e) => Err(e),
    }
}
