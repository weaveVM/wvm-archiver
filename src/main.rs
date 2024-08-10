use crate::utils::get_block::by_number;
use crate::utils::transaction::test_send;
use crate::utils::schema::Block;

mod utils;
#[tokio::main]
async fn main() {
    let block = by_number(123820363).await;
    println!("{:?}", block.as_ref().unwrap());
    let x = serde_json::json!(block.as_ref().unwrap());
    println!("{:?}", x);
    let y = Block::load_block_from_value(x).unwrap();
    let borsh_res = Block::borsh_ser(&y);
    let brotli_res = Block::brotli_compress(&borsh_res);
    println!("borsh vec length: {:?}", borsh_res.len());
    println!("brotli vec length: {:?}", brotli_res.len());

    let _ = test_send(brotli_res).await;
    // if let Ok(Some(block_result)) = block {
    //     println!("{:?}", block_result);
    // }
}
