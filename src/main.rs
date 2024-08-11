use crate::utils::archive_block::archive;
use crate::utils::schema::Network;
use crate::utils::planetscale::{ps_archive_block, ps_get_latest_block_id};
use std::thread;
use std::time::Duration;

mod utils;
#[tokio::main]
async fn main() {
    let network = Network::config();
    let block_time = network.block_time;
    let ps_latest_archived_block = ps_get_latest_block_id().await;
    // it defaults to network.start_block if planestcale fails
    let mut start_block = ps_latest_archived_block;

    println!("\n{:#?}\n\n", network);

    // poll blocks & archive
    loop {
        println!("\n{}", "#".repeat(100));
        println!(
            "\nARCHIVING BLOCK #{} of Network {} -- ChainId: {}\n",
            start_block, network.name, network.network_chain_id
        );
        let archive_txid = archive(Some(start_block)).await.unwrap();
        let _ =  ps_archive_block(&start_block, &archive_txid).await;
        start_block += 1;
        println!("\n{}", "#".repeat(100));
        thread::sleep(Duration::from_secs(block_time.into()));
    }
}
