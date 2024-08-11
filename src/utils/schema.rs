use crate::utils::env_var::get_env_var;
use borsh::to_vec;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use ethers_providers::{Http, Provider};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    pub name: String,
    pub network_chain_id: u32,
    pub wvm_chain_id: u32,
    pub network_rpc: String,
    pub wvm_rpc: String,
    pub block_time: u32,
    pub start_block: u64, // as per ethers_provider
    pub archiver_address: String,
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

#[derive(Debug, Deserialize, Serialize, BorshSerialize, BorshDeserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub base_fee_per_gas: Option<String>,         // "baseFeePerGas"
    pub blob_gas_used: Option<String>,            // "blobGasUsed"
    pub difficulty: String,                       // "difficulty"
    pub excess_blob_gas: Option<String>,          // "excessBlobGas"
    pub extra_data: String,                       // "extraData"
    pub gas_limit: String,                        // "gasLimit"
    pub gas_used: String,                         // "gasUsed"
    pub hash: String,                             // "hash"
    pub logs_bloom: String,                       // "logsBloom"
    pub miner: String,                            // "miner"
    pub mix_hash: String,                         // "mixHash"
    pub nonce: String,                            // "nonce"
    pub number: String,                           // "number"
    pub parent_beacon_block_root: Option<String>, // "parentBeaconBlockRoot"
    pub parent_hash: String,                      // "parentHash"
    pub receipts_root: String,                    // "receiptsRoot"
    pub seal_fields: Vec<String>,                 // "sealFields" as an array of strings
    pub sha3_uncles: String,                      // "sha3Uncles"
    pub size: String,                             // "size"
    pub state_root: String,                       // "stateRoot"
    pub timestamp: String,                        // "timestamp"
    pub total_difficulty: String,                 // "totalDifficulty"
    pub transactions: Vec<String>,                // "transactions" as an array of strings
}

impl Block {
    pub fn load_block_from_value(value: Value) -> Result<Block, serde_json::Error> {
        serde_json::from_value::<Block>(value)
    }
    pub fn brotli_compress(input: &[u8]) -> Vec<u8> {
        let mut writer = brotli::CompressorWriter::new(Vec::new(), 4096, 11, 22);
        writer.write_all(input).unwrap();
        writer.into_inner()
    }
    pub fn borsh_ser(input: &Block) -> Vec<u8> {
        to_vec(input).unwrap()
    }
}
