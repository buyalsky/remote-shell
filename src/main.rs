use std::env;
use std::process::exit;

use crate::client::initialize_client;
use crate::config::{ClientConfig, ServerConfig};
use crate::server::initialize_server;

mod config;
mod server;
mod client;

const AUTHENTICATED: &[u8] = "Authenticated".as_bytes();
const FAILED_AUTHENTICATION: &[u8] = "Failed".as_bytes();
const NULL_RESPONSE: &[u8] = " ".as_bytes();

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        7 => initialize_server(ServerConfig::parse_server_config(args)),
        9 => initialize_client(ClientConfig::parse_client_config(args)),
        _ => {
            println!("Usage for server: -p port -u username -s password");
            println!("Usage for client: -h host_address -p port -u username -s password");
            exit(1);
        }
    };
}
