# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## About the Project
WeaveVM Archiver is a Rust-based ETL archive pipeline for EVM networks that archives block data to WeaveVM. It acts as a bridge between existing EVM networks and WeaveVM's permanent data storage, allowing developers to interface with WeaveVM without smart contract redeployments.

## Development Commands

### Building and Running
- `cargo run` - Build and run the application locally
- `cargo build` - Build the project
- `cargo check` - Check for compilation errors without building
- `cargo test` - Run tests (if any exist)

### Docker Development
- `docker-compose up -d` - Start eRPC proxy for RPC caching/load balancing
- Modify `erpc.yaml` for RPC configuration

## Architecture Overview

### Core Components
- **Main Application** (`src/main.rs`): Axum-based web server with parallel block archiving
- **Block Archiving** (`src/utils/archive_block.rs`): Core archiving logic that fetches, serializes, and archives EVM blocks
- **Schema Definitions** (`src/utils/schema.rs`): Data structures for Network config, Block data, and server responses
- **Server Handlers** (`src/utils/server_handlers.rs`): RESTful API endpoints for retrieving archived data
- **Database Integration** (`src/utils/planetscale.rs`): PlanetScale cloud database operations
- **Transaction Management** (`src/utils/transaction.rs`): WeaveVM transaction handling

### Data Flow
1. Fetches EVM block data from configured network RPC
2. Serializes block data using Borsh format
3. Compresses serialized data with Brotli compression
4. Submits compressed data to WeaveVM as calldata transactions
5. Indexes block ID to WeaveVM TXID mapping in PlanetScale database

### Network Configuration
- Network configs are stored in `networks/` directory as JSON files
- Each network requires: RPC endpoints, chain IDs, archiver addresses, block timing
- Configuration loaded from environment variable `network` pointing to config file path

## Environment Setup
- Copy and configure environment variables (see `README.md` for required vars)
- Set up PlanetScale database using `db_schema.sql`
- Fund archiver address with tWVM tokens for WeaveVM transactions

## Server API Endpoints
- `GET /info` - Node instance information and status
- `GET /block/:id` - Retrieve WeaveVM archive TXID for given EVM block ID  
- `GET /block/raw/:id` - Decode and return original block data as JSON
- `GET /` - Health check endpoint

## Key Dependencies
- **Axum**: Web framework for API endpoints and HTTP server
- **Ethers**: Ethereum RPC client and utilities
- **Borsh**: Binary serialization format
- **Brotli**: Compression algorithm
- **PlanetScale**: MySQL-compatible serverless database