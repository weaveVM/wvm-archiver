use crate::utils::{
    env_var::get_env_var, get_block::get_current_block_number,
    planetscale::ps_get_archived_blocks_count, transaction::get_balance_of,
};
use ethers::types::U256;
use ethers_providers::{Http, Provider};
use planetscale_driver::Database;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    pub name: String,
    pub network_chain_id: u32,
    pub wvm_chain_id: u32,
    pub network_rpc: String,
    pub wvm_rpc: String,
    pub block_time: f32,
    pub start_block: u64,
    pub archiver_address: String,
    pub backfill_address: String,
    pub archive_pool_address: String,
}

impl Network {
    pub fn config() -> Network {
        let network_config = get_env_var("network").unwrap();
        let mut file = File::open(network_config).unwrap();
        let mut data = String::new();

        file.read_to_string(&mut data).unwrap();

        let network: Network = serde_json::from_str(&data).unwrap();
        // cannot self send data
        assert_ne!(network.archiver_address, network.archive_pool_address);
        network
    }

    pub async fn provider(&self, rpc: bool) -> Provider<Http> {
        let target_rpc: &String;

        let network: Network = Self::config();
        if rpc {
            target_rpc = &network.wvm_rpc;
        } else {
            target_rpc = &network.network_rpc
        }
        let provider: Provider<Http> =
            Provider::<Http>::try_from(target_rpc).expect("could not instantiate HTTP Provider");

        provider
    }
}

#[derive(Database, Debug, Serialize)]
pub struct PsGetBlockTxid {
    pub wvm_archive_txid: String,
}

#[derive(Database, Debug, Serialize)]
pub struct PsGetExtremeBlock {
    pub block_id: u64,
}

#[derive(Database, Debug, Serialize)]
pub struct PsGetTotalBlocksCount {
    pub count: u64,
}

#[derive(Debug, Serialize)]
pub struct InfoServerResponse {
    first_livesync_archived_block: Option<u64>,
    last_livesync_archived_block: Option<u64>,
    first_backfill_archived_block: Option<u64>,
    last_backfill_archived_block: Option<u64>,
    livesync_start_block: u64,
    total_archived_blocks: u64,
    blocks_behind_live_blockheight: u64,
    archiver_balance: U256,
    archiver_address: String,
    backfill_address: String,
    backfill_balance: U256,
    network_name: String,
    network_chain_id: u32,
    network_rpc: String,
}

impl InfoServerResponse {
    pub async fn new(
        first_livesync_block: Option<u64>,
        last_livesync_block: Option<u64>,
        first_backfill_block: Option<u64>,
        last_backfill_block: Option<u64>,
    ) -> InfoServerResponse {
        let network = Network::config();
        // balances
        let archiver_balance = get_balance_of(network.archiver_address.clone()).await;
        let archiver_balance = Some(archiver_balance).unwrap_or("0".into());
        let backfill_balance = get_balance_of(network.backfill_address.clone()).await;
        let backfill_balance = Some(backfill_balance).unwrap_or("0".into());
        // blocks stats
        let total_archived_blocks = ps_get_archived_blocks_count().await;
        let current_live_block = get_current_block_number().await.as_u64();
        let blocks_behind_live_blockheight = current_live_block - last_livesync_block.unwrap_or(0);

        let instance: InfoServerResponse = InfoServerResponse {
            archiver_balance,
            backfill_balance,
            blocks_behind_live_blockheight,
            livesync_start_block: network.start_block,
            first_livesync_archived_block: first_livesync_block,
            first_backfill_archived_block: first_backfill_block,
            last_livesync_archived_block: last_livesync_block,
            last_backfill_archived_block: last_backfill_block,
            total_archived_blocks,
            archiver_address: network.archiver_address,
            backfill_address: network.backfill_address,
            network_name: network.name,
            network_chain_id: network.network_chain_id,
            network_rpc: network.network_rpc,
        };
        instance
    }
}
