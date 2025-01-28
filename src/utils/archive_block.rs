use crate::utils::env_var::get_env_var;
use crate::utils::get_block::{
    get_block_by_number, get_current_block_number, WvmArchiverDataBlock,
};
use crate::utils::planetscale::{ps_archive_block, ps_get_latest_block_id};
use crate::utils::schema::Network;
use crate::utils::transaction::{send_wvm_calldata, send_wvm_calldata_backfill};
use std::{thread, time::Duration};

pub async fn archive(
    block_number: Option<u64>,
    is_backfill: bool,
) -> Result<String, anyhow::Error> {
    let network = Network::config();
    let block_to_archive = block_number.unwrap_or(if is_backfill {
        get_env_var("backfill_start_block")
            .unwrap()
            .parse::<u64>()
            .unwrap()
    } else {
        network.start_block
    });

    // assert to ensure backfill and livesync dont collide at the continuity checkpoint
    if is_backfill {
        assert!(block_number.unwrap() < network.start_block)
    }
    // fetch block
    let block_data = get_block_by_number(block_to_archive)
        .await
        .map_err(|(status, msg)| anyhow::anyhow!("Block error ({}): {}", status, msg))?;
    // serialize response into Block struct
    let borsh_res = WvmArchiverDataBlock::borsh_ser(&block_data);
    // brotli compress the borsh serialized block
    let brotli_res = WvmArchiverDataBlock::brotli_compress(&borsh_res);

    let txid = if is_backfill {
        send_wvm_calldata_backfill(brotli_res).await?
    } else {
        send_wvm_calldata(brotli_res).await?
    };

    Ok(txid)
}

pub async fn sprint_blocks_archiving(is_backfill: bool) -> Result<(), anyhow::Error> {
    let network = Network::config();
    let block_time = network.block_time;
    let mut current_block_number = get_current_block_number().await.as_u64();
    // it defaults to network.start_block or env.backfill_start_block
    // based on is_backfill if planestcale fails
    let mut start_block = ps_get_latest_block_id(is_backfill).await;

    loop {
        if start_block < current_block_number - 1 {
            println!("\n{}", "#".repeat(100));
            println!(
                "\nARCHIVING BLOCK #{} of Network {} -- ChainId: {} -- IS_BACKFILL: {}\n",
                start_block, network.name, network.network_chain_id, is_backfill
            );
            let archive_txid = archive(Some(start_block), is_backfill)
                .await
                .unwrap_or_default();
            let _ = ps_archive_block(&start_block, &archive_txid, is_backfill)
                .await
                .unwrap_or_default();
            start_block += 1;
            println!("\n{}", "#".repeat(100));
        } else {
            current_block_number = get_current_block_number().await.as_u64();
            thread::sleep(Duration::from_secs(block_time as u64));
        }
    }
}
