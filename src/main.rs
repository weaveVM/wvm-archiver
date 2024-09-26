use crate::utils::archive_block::sprint_blocks_archiving;
use crate::utils::backfill_genesis::backfill_from_genesis;
use crate::utils::schema::Network;
use crate::utils::server_handlers::{handle_block, handle_block_raw, handle_info, handle_weave_gm};
use axum::{routing::get, Router};
use tokio::task;

mod utils;
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let network = Network::config();

    println!("\n{:#?}\n\n", network);
    // server routes
    let router = Router::new()
        .route("/", get(handle_weave_gm))
        .route("/info", get(handle_info))
        .route("/block/:id", get(handle_block))
        .route("/block/raw/:id", get(handle_block_raw));

    // poll blocks & sprint archiving in parallel
    task::spawn(async move {
        sprint_blocks_archiving().await;
    });
    // backfill blocks from genesis till network.start_block
    task::spawn(async move {
        backfill_from_genesis().await.unwrap();
    });
    Ok(router.into())
}
