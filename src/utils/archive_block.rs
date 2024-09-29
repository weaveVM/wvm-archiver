use crate::utils::get_block::{by_number, get_current_block_number};
use crate::utils::planetscale::{ps_archive_block, ps_get_latest_block_id};
use crate::utils::schema::{Block, Network};
use crate::utils::transaction::{send_wvm_calldata, send_wvm_calldata_backfill};
use anyhow::Error;
use std::{thread, time::Duration};

pub async fn archive(block_number: Option<u64>, is_backfill: bool) -> Result<String, Error> {
    let network = Network::config();
    let start_block = network.start_block;
    let block_to_archive = block_number.unwrap_or(start_block);

    // fetch block
    let block_data = by_number(block_to_archive).await.unwrap();
    // serialize response into Block struct
    let block_data_json = serde_json::json!(block_data.as_ref().unwrap());
    let block_data_struct = Block::load_block_from_value(block_data_json).unwrap();
    // borsh serialize the struct
    let borsh_res = Block::borsh_ser(&block_data_struct);
    // brotli compress the borsh serialized block
    let brotli_res = Block::brotli_compress(&borsh_res);

    // println!("borsh vec length: {:?}", borsh_res.len());
    // println!("brotli vec length: {:?}", brotli_res.len());

    let txid = if is_backfill {
        send_wvm_calldata_backfill(brotli_res).await.unwrap()
    } else {
        send_wvm_calldata(brotli_res).await.unwrap()
    };

    Ok(txid)
}

pub async fn sprint_blocks_archiving() {
    let network = Network::config();
    let block_time = network.block_time;
    let mut current_block_number = get_current_block_number().await.as_u64();
    let ps_latest_archived_block = ps_get_latest_block_id().await;
    // it defaults to network.start_block if planestcale fails
    let mut start_block = if ps_latest_archived_block < network.start_block {
        network.start_block
    } else {
        ps_latest_archived_block
    };

    loop {
        if start_block < current_block_number - 1 {
            println!("\n{}", "#".repeat(100));
            println!(
                "\nARCHIVING BLOCK #{} of Network {} -- ChainId: {}\n",
                start_block, network.name, network.network_chain_id
            );
            let archive_txid = archive(Some(start_block), false).await.unwrap();
            let _ = ps_archive_block(&start_block, &archive_txid).await;
            start_block += 1;
            println!("\n{}", "#".repeat(100));
        } else {
            current_block_number = get_current_block_number().await.as_u64();
            thread::sleep(Duration::from_secs(block_time as u64));
        }
    }
}
