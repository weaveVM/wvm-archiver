use crate::utils::planetscale::{ps_get_archived_block_txid, ps_get_blocks_extremes};
use crate::utils::schema::InfoServerResponse;
use crate::utils::transaction::decode_wvm_tx_data;
use crate::utils::schema::Block;
use axum::{extract::Path, response::Json};
use serde::de::value;
use serde_json::Value;

pub async fn handle_weave_gm() -> &'static str {
    "WeaveGM!"
}

pub async fn handle_block(Path(id): Path<u64>) -> Json<Value> {
    let txid = ps_get_archived_block_txid(id).await;
    Json(txid)
}

pub async fn handle_info() -> Json<Value> {
    let first = ps_get_blocks_extremes("first").await;
    let last = ps_get_blocks_extremes("last").await;

    let first_block = first.get("block_id").unwrap().as_u64();
    let last_block = last.get("block_id").unwrap().as_u64();
    let stats_res = InfoServerResponse::new(first_block, last_block).await;

    let res = serde_json::to_value(&stats_res).unwrap();
    Json(res)
}

pub async fn handle_block_raw(Path(id): Path<u64>) -> Json<Value> {
    let tx_object = ps_get_archived_block_txid(id).await;
    let txid = &tx_object["wvm_archive_txid"].as_str().unwrap();
    let decoded_block: Block = decode_wvm_tx_data(txid).await;
    let res = serde_json::to_value(&decoded_block).unwrap();
    Json(res)
}