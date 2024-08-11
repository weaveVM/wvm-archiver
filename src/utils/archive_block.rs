use crate::utils::get_block::by_number;
use crate::utils::schema::{Block, Network};
use crate::utils::transaction::send_wvm_calldata;

pub async fn archive(block_number: Option<u64>) {
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

    let _ = send_wvm_calldata(brotli_res).await;
}
