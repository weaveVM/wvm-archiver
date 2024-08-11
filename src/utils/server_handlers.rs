use crate::utils::planetscale::ps_get_archived_block_txid;
use serde_json::Value;
use axum::{
    extract::Path,
    response::Json
};

pub async fn weave_gm() -> &'static str {
    "WeaveGM!"
}

pub async fn handle_block(Path(id): Path<u64>) -> Json<Value> {
    let txid = ps_get_archived_block_txid(id).await;
    Json(txid)
}

