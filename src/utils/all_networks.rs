use crate::utils::schema::Network;
use serde_json::{json, Value};
use std::{fs::File, io::Read};

static ALL_NETWORKS: [&str; 11] = [
    "goat",
    "metis",
    "rss3",
    "sei",
    "humanode",
    "dymension",
    "avalanche",
    "monad",
    "scroll",
    "phala",
    "rise"
];

pub async fn get_all_networks_metadata() -> Value {
    let mut networks_vec: Vec<Network> = Vec::new();

    for &network in ALL_NETWORKS.iter() {
        let network_path = format!("./networks/{}.json", &network);
        let mut file = File::open(network_path).unwrap();
        let mut data = String::new();

        file.read_to_string(&mut data).unwrap();

        let evm_network: Network = serde_json::from_str(&data).unwrap();
        networks_vec.push(evm_network);
    }

    json!(networks_vec)
}
