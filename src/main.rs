use crate::utils::get_block::by_number;
use crate::utils::transaction::test_send;
use borsh::to_vec;
use crate::utils::schema::Block;

mod utils;
#[tokio::main]
async fn main() {
    // test_send().await;
    let block = by_number(20490776).await;
    // let res = Some(Block {block.unwrap()});
    println!("{:?}", block.as_ref().unwrap());
    let x = serde_json::json!(block.as_ref().unwrap());
    println!("{:?}", x);
    let y = Block::load_block_from_value(x).unwrap();
    let borsh_res = to_vec(&y).unwrap();
    let brotli_res = Block::brotli_compress(&borsh_res);
    println!("{:?}", brotli_res);
    let abc = vec![1,2];
    let _ = test_send(brotli_res).await;
    // if let Ok(Some(block_result)) = block {
    //     println!("{:?}", block_result);
    // }
}
