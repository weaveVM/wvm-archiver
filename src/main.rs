use ethers_core::k256::pkcs8::der::asn1::Null;

use crate::utils::archive_block::archive;
use crate::utils::schema::Network;

use std::thread;
use std::time::Duration;

mod utils;
#[tokio::main]
async fn main() {
    let network = Network::config();
    let block_time = network.block_time;
    let mut start_block = network.start_block;

    println!("\n{:#?}\n\n", network);

    // poll block & archive
    loop {
        println!("\n{}", "#".repeat(100));
        println!("\nARCHIVING BLOCK #{} of Network {} -- ChainId: {}\n", start_block, network.name, network.network_chain_id);
        archive(Some(start_block)).await;
        start_block += 1;
        println!("\n{}", "#".repeat(100));
        thread::sleep(Duration::from_secs(block_time.into()));
    }
}
