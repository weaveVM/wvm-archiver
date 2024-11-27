use crate::utils::env_var::get_env_var;
use crate::utils::schema::{Network, PsGetBlockTxid, PsGetExtremeBlock, PsGetTotalBlocksCount};
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
    is_backfill: bool
) -> Result<(), Error> {
    // format to the table VAR(66) limitation
    let wvm_calldata_txid = wvm_calldata_txid.trim_matches('"');
    let conn = ps_init().await;
    let mut ps_table_name = get_env_var("ps_table_name").unwrap();

    if is_backfill {
        ps_table_name = format!("{}{}", ps_table_name, "Backfill")
    }

    let query_str = format!(
        "INSERT INTO {}(NetworkBlockId, WeaveVMArchiveTxid) VALUES($0, \"$1\")",
        ps_table_name
    );


    let res = query(
        &query_str,
    )
    .bind(network_block_id)
    .bind(wvm_calldata_txid)
    .execute(&conn)
    .await;

    match res {
        Ok(result) => {
            println!("Insert operation was successful: {:?}", result);
            Ok(result)
        }
        Err(e) => {
            println!("Error occurred during insert operation: {:?}", e);
            Err(e)
        }
    }
}

pub async fn ps_get_latest_block_id(is_backfill: bool) -> u64 {
    let network = Network::config();
    let conn = ps_init().await;

    let mut ps_table_name = get_env_var("ps_table_name").unwrap();
    if is_backfill {
        ps_table_name = format!("{}{}", ps_table_name, "Backfill")
    }

    let query_str = format!(
        "SELECT MAX(NetworkBlockId) AS LatestNetworkBlockId FROM {};",
        ps_table_name
    );

    let default_start_block = if is_backfill {
        get_env_var("backfill_start_block").unwrap().parse::<u64>().unwrap()
    } else {
        network.start_block
    };

    let latest_archived: u64 =
        query(&query_str)
            .fetch_scalar(&conn)
            .await
            .unwrap_or(default_start_block);
    // return latest archived block in planetscale + 1
    // so the process can start archiving from latest_archived + 1
    latest_archived + 1
}

pub async fn ps_get_archived_block_txid(id: u64) -> Value {
    let conn = ps_init().await;

    let query_formatted = format!(
        "SELECT WeaveVMArchiveTxid FROM WeaveVMArchiverMetis WHERE NetworkBlockId = {}",
        id
    );
    let txid: PsGetBlockTxid = query(&query_formatted).fetch_one(&conn).await.unwrap();

    let res = serde_json::json!(txid);
    res
}

pub async fn ps_get_blocks_extremes(extreme: &str) -> Value {
    let conn = ps_init().await;

    let query_type = match extreme {
        "first" => "ASC",
        "last" => "DESC",
        _ => panic!("invalid extreme value. Use 'first' or 'last'."),
    };

    let query_formatted = format!(
        "SELECT NetworkBlockId FROM WeaveVMArchiverMetis ORDER BY NetworkBlockId {} LIMIT 1;",
        query_type
    );

    let query: PsGetExtremeBlock = query(&query_formatted).fetch_one(&conn).await.unwrap();

    let res = serde_json::json!(query);
    res
}

pub async fn ps_get_archived_blocks_count() -> PsGetTotalBlocksCount {
    let conn = ps_init().await;

    let query_formatted = "SELECT MAX(Id) FROM WeaveVMArchiverMetis;";
    let count: PsGetTotalBlocksCount = query(&query_formatted).fetch_one(&conn).await.unwrap();
    count
}
