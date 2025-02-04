pub mod client;

pub mod network;


fn test () {
    client::connect();

    network::network_connect();

    network::server::server_connect();
}