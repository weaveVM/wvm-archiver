use crate::utils::schema::Network;
use axum::http::StatusCode;
use borsh::{from_slice, to_vec};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use ethers_core::types::U64;
use ethers_providers::Middleware;
use evm_state_reconstructing::utils::core::evm_wvm_types::{
    WvmBlock, WvmTransaction, WvmTransactionReceipt,
};
use serde::Serialize;
use std::io::{Read, Write};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize)]
pub struct WvmArchiverDataBlock {
    pub block: Option<WvmBlock<WvmTransaction>>,
    pub txs_receipts: Option<Vec<WvmTransactionReceipt>>,
}

impl WvmArchiverDataBlock {
    pub fn new() -> Self {
        Self {
            block: None,
            txs_receipts: None,
        }
    }

    pub fn from(
        block: Option<WvmBlock<WvmTransaction>>,
        txs_receipts: Option<Vec<WvmTransactionReceipt>>,
    ) -> Self {
        Self {
            block,
            txs_receipts,
        }
    }
}

impl WvmArchiverDataBlock {
    pub fn brotli_compress(input: &[u8]) -> Vec<u8> {
        let mut writer = brotli::CompressorWriter::new(Vec::new(), 4096, 11, 22);
        writer.write_all(input).unwrap();
        writer.into_inner()
    }
    pub fn brotli_decompress(input: Vec<u8>) -> Vec<u8> {
        let mut decompressed_data = Vec::new();
        let mut decompressor = brotli::Decompressor::new(input.as_slice(), 4096); // 4096 is the buffer size

        decompressor
            .read_to_end(&mut decompressed_data)
            .expect("Decompression failed");
        decompressed_data
    }
    pub fn borsh_ser(input: &WvmArchiverDataBlock) -> Vec<u8> {
        to_vec(input).unwrap()
    }
    pub fn borsh_der(input: Vec<u8>) -> WvmArchiverDataBlock {
        let res: WvmArchiverDataBlock =
            from_slice(&input).expect("error deseriliazing the calldata");
        res
    }
}

pub async fn get_block_by_number(
    number: u64,
) -> Result<WvmArchiverDataBlock, (StatusCode, &'static str)> {
    let network: Network = Network::config();

    let provider = Network::provider(&network, false).await;
    let block = provider
        .get_block_with_txs(number)
        .await
        .unwrap_or_default();
    if let Some(block) = block {
        let wvm_block: WvmBlock<WvmTransaction> = WvmBlock::from(block.clone());
        let mut wvm_receipts: Vec<WvmTransactionReceipt> = vec![];

        for tx in &block.transactions {
            if let Some(receipt) = provider
                .get_transaction_receipt(tx.hash)
                .await
                .unwrap_or_default()
            {
                wvm_receipts.push(receipt.into());
            } else {
                println!("Receipt not found for transaction: {:?}", tx.hash);
            }
        }

        let wvm_archiver_data_block =
            WvmArchiverDataBlock::from(Some(wvm_block), Some(wvm_receipts));
        Ok(wvm_archiver_data_block)
    } else {
        Err((StatusCode::BAD_REQUEST, "Block not found"))
    }
}

pub async fn get_current_block_number() -> U64 {
    let network: Network = Network::config();
    // connect to the target EVM provider
    let provider = Network::provider(&network, false).await;
    let block_number = provider.get_block_number().await.unwrap_or_default();
    block_number
}
