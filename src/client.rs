use std::hash::{Hash, Hasher};
use std::net::SocketAddr;
use chrono::*;

pub struct Client {
	addr: SocketAddr,
	last_ack: DateTime<UTC>,
}

impl Hash for Client {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.addr.hash(state)
    }
}

impl PartialEq for Client {
    fn eq(&self, other: &Client) -> bool {
        self.addr.eq(&other.addr)
    }
}

impl Eq for Client {}

impl Client {
	pub fn new(address: SocketAddr) -> Client {
		Client {
			addr: address,
			last_ack: UTC::now(),
		}
	}
}