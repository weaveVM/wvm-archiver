use crate::utils::all_networks::get_all_networks_metadata;
use crate::utils::get_block::WvmArchiverDataBlock;
use crate::utils::planetscale::{ps_get_archived_block_txid, ps_get_blocks_extremes};
use crate::utils::schema::InfoServerResponse;
use crate::utils::transaction::decode_wvm_tx_data;
use axum::{extract::Path, response::Json};
use serde_json::Value;

pub async fn handle_weave_gm() -> &'static str {
    "WeaveGM!"
}

pub async fn handle_block(Path(id): Path<u64>) -> Json<Value> {
    let txid = ps_get_archived_block_txid(id).await;
    Json(txid)
}

pub async fn handle_info() -> Json<Value> {
    let first_livesync = ps_get_blocks_extremes("first", false).await;
    let last_livesync = ps_get_blocks_extremes("last", false).await;
    let first_backfill = ps_get_blocks_extremes("first", true).await;
    let last_backfill = ps_get_blocks_extremes("last", true).await;

    let first_livesync = first_livesync.get("block_id").unwrap().as_u64();
    let last_livesync = last_livesync.get("block_id").unwrap().as_u64();
    let first_backfill = first_backfill.get("block_id").unwrap().as_u64();
    let last_backfill = last_backfill.get("block_id").unwrap().as_u64();
    let stats_res: InfoServerResponse =
        InfoServerResponse::new(first_livesync, last_livesync, first_backfill, last_backfill).await;

    let res = serde_json::to_value(&stats_res).unwrap();
    Json(res)
}

pub async fn handle_block_raw(Path(id): Path<u64>) -> Json<Value> {
    let tx_object = ps_get_archived_block_txid(id).await;
    let txid = &tx_object["wvm_archive_txid"].as_str().unwrap();
    let decoded_block: WvmArchiverDataBlock = decode_wvm_tx_data(txid).await;
    let res = serde_json::to_value(&decoded_block).unwrap();
    Json(res)
}

pub async fn handle_all_networks_info() -> Json<Value> {
    let all_networks = get_all_networks_metadata().await;
    Json(all_networks)
}
