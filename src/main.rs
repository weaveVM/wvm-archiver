use crate::utils::archive_block::sprint_blocks_archiving;
use crate::utils::schema::Network;
use crate::utils::server_handlers::{
    handle_all_networks_info, handle_block, handle_block_raw, handle_info, handle_weave_gm,
};
use axum::{routing::get, Router};
use tokio::task;
use tokio::net::TcpListener;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let network = Network::config();

    println!("\n{:#?}\n\n", network);
    
    // server routes
    let router = Router::new()
        .route("/", get(handle_weave_gm))
        .route("/v1/info", get(handle_info))
        .route("/v1/block/{id}", get(handle_block))
        .route("/v1/block/raw/{id}", get(handle_block_raw))
        .route("/v1/all-networks-info", get(handle_all_networks_info));

    // poll blocks & sprint archiving in parallel
    task::spawn(async move {
        let _ = sprint_blocks_archiving(false).await.unwrap_or_default();
    });
    // backfill blocks from genesis till network.start_block
    task::spawn(async move {
        let _ = sprint_blocks_archiving(true).await.unwrap_or_default();
    });

    // Get port from environment variable or default to 3000
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    println!("Server starting on {}", addr);
    
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;
    
    Ok(())
}
