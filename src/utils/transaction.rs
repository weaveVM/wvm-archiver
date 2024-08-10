use crate::utils::schema::Network;
use crate::utils::env_var::get_env_var;
use ethers_providers::{Provider, Http};
use ethers::{utils, prelude::*};

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

pub async fn send_wvm_calldata(block_data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let network = Network::config();
    let provider = Network::provider(&network, true).await;
    let private_key = get_env_var("archiver_pk").unwrap();
    let wallet: LocalWallet = private_key
        .parse::<LocalWallet>()?
        .with_chain_id(network.wvm_chain_id); 
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    let address_from = network.archiver_address.parse::<Address>()?;
    let address_to = network.archive_pool_address.parse::<Address>()?;
    
    assert_non_zero_balance(&provider, &address_from).await;
    send_transaction(&client, &address_from, &address_to, block_data).await?;

    Ok(())
}

async fn assert_non_zero_balance(provider: &Provider<Http>, address: &Address) {
    let balance = provider.get_balance(address.clone(), None).await.unwrap();
    assert!(balance > 0.into());
}

async fn send_transaction(client: &Client, address_from: &Address, address_to: &Address, block_data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "\nArchiving block data from archiver: {} to archive pool: {}",
        address_from, address_to
    );
    let tx = TransactionRequest::new()
        .to(address_to.clone())
        .value(U256::from(utils::parse_ether(0)?))
        .from(address_from.clone())
        .data(block_data);

    let tx = client.send_transaction(tx, None).await?.await?;
    let json_tx = serde_json::json!(tx);
    println!("\nWeaveVM Archiving TXID: {}", json_tx["transactionHash"]);

    Ok(())
}

