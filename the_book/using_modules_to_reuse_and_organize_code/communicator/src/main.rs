// extern crate communicator;
use communicator::{client, network};
fn main () {
   client::connect();
   network::server::server_connect();
}