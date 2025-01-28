use crate::utils::{
    env_var::get_env_var,
    schema::{Network, PsGetBlockTxid, PsGetExtremeBlock, PsGetTotalBlocksCount},
};
use anyhow::Error;
use planetscale_driver::{query, PSConnection};
use serde_json::Value;

async fn ps_init() -> PSConnection {
    let host = get_env_var("DATABASE_HOST").unwrap();
    let username = get_env_var("DATABASE_USERNAME").unwrap();
    let password = get_env_var("DATABASE_PASSWORD").unwrap();

    let conn: PSConnection = PSConnection::new(&host, &username, &password);

    conn
}

pub async fn ps_archive_block(
    network_block_id: &u64,
    wvm_calldata_txid: &str,
    is_backfill: bool,
) -> Result<(), Error> {
    // format to the table VAR(66) limitation
    let wvm_calldata_txid = wvm_calldata_txid.trim_matches('"');
    let conn = ps_init().await;
    let mut ps_table_name = get_env_var("ps_livesync_table_name").unwrap();

    if is_backfill {
        ps_table_name = get_env_var("ps_backfill_table_name").unwrap();
    }

    let query_str = format!(
        "INSERT INTO {}(NetworkBlockId, WeaveVMArchiveTxid) VALUES($0, \"$1\")",
        ps_table_name
    );

    let res = query(&query_str)
        .bind(network_block_id)
        .bind(wvm_calldata_txid)
        .execute(&conn)
        .await?;

    println!("Insert operation was successful: {:?}", res);
    Ok(res)
}

pub async fn ps_get_latest_block_id(is_backfill: bool) -> u64 {
    let network = Network::config();
    let conn = ps_init().await;

    let mut ps_table_name = get_env_var("ps_livesync_table_name").unwrap();
    if is_backfill {
        ps_table_name = get_env_var("ps_backfill_table_name").unwrap();
    }

    let query_str = format!(
        "SELECT MAX(NetworkBlockId) AS LatestNetworkBlockId FROM {};",
        ps_table_name
    );

    let default_start_block = if is_backfill {
        get_env_var("backfill_start_block")
            .unwrap()
            .parse::<u64>()
            .unwrap()
    } else {
        network.start_block
    };

    let latest_archived: u64 = query(&query_str)
        .fetch_scalar(&conn)
        .await
        .unwrap_or(default_start_block);
    // return latest archived block in planetscale + 1
    // so the process can start archiving from latest_archived + 1
    latest_archived + 1
}

pub async fn ps_get_archived_block_txid(id: u64) -> Value {
    let conn = ps_init().await;
    let network = Network::config();
    let ps_livesync_table_name = get_env_var("ps_livesync_table_name").unwrap();
    let ps_backfill_table_name = get_env_var("ps_backfill_table_name").unwrap();

    let query_formatted_livesync = format!(
        "SELECT WeaveVMArchiveTxid FROM {} WHERE NetworkBlockId = {}",
        ps_livesync_table_name, id
    );

    let query_formatted_backfill = format!(
        "SELECT WeaveVMArchiveTxid FROM {} WHERE NetworkBlockId = {}",
        ps_backfill_table_name, id
    );

    // query from tables based on block id existence in the livesync of backfill
    let query_formatted = if id >= network.start_block {
        query_formatted_livesync
    } else {
        query_formatted_backfill
    };

    let txid: PsGetBlockTxid = query(&query_formatted).fetch_one(&conn).await.unwrap();

    let res = serde_json::json!(txid);
    res
}

pub async fn ps_get_blocks_extremes(extreme: &str, is_backfill: bool) -> Value {
    let conn = ps_init().await;

    let mut ps_table_name = get_env_var("ps_livesync_table_name").unwrap();

    ps_table_name = if is_backfill {
        get_env_var("ps_backfill_table_name").unwrap()
    } else {
        ps_table_name
    };

    let query_type = match extreme {
        "first" => "ASC",
        "last" => "DESC",
        _ => panic!("invalid extreme value. Use 'first' or 'last'."),
    };

    let query_formatted = format!(
        "SELECT NetworkBlockId FROM {} ORDER BY NetworkBlockId {} LIMIT 1;",
        ps_table_name, query_type
    );

    let query: PsGetExtremeBlock = query(&query_formatted).fetch_one(&conn).await.unwrap();

    let res = serde_json::json!(query);
    res
}

pub async fn ps_get_archived_blocks_count() -> u64 {
    let conn = ps_init().await;
    let ps_livesync_table_name = get_env_var("ps_livesync_table_name").unwrap();
    let ps_backfill_table_name = get_env_var("ps_backfill_table_name").unwrap();

    let query_formatted_livesync = format!("SELECT MAX(Id) FROM {};", ps_livesync_table_name);
    let query_formatted_backfill = format!("SELECT MAX(Id) FROM {};", ps_backfill_table_name);
    let count_livesync: PsGetTotalBlocksCount = query(&query_formatted_livesync)
        .fetch_one(&conn)
        .await
        .unwrap();
    let count_backfill: PsGetTotalBlocksCount = query(&query_formatted_backfill)
        .fetch_one(&conn)
        .await
        .unwrap();
    count_livesync.count + count_backfill.count
}
