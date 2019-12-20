//! netmod-udp is a UDP overlay for RATMAN

use async_std::net::UdpSocket;
use identity::Identity;
use std::{
    collections::{BTreeMap, BTreeSet},
    net::IpAddr,
    sync::{Arc, Mutex},
};

enum UdpCommand {
    /// Used to announce a netmod endpoint via broadcasts
    Announce,
    /// Send an ID announcement to known UDP endpoints
    Id(Identity),
}

/// An internal envelope that is used as a transfer protocol
enum UdpEnvelope {
    /// A tunneled data payload
    Data(Vec<u8>),
    /// An overlay command payload
    Internal(UdpCommand),
}

/// Represents an IP network tunneled via UDP
pub struct Endpoint {
    ips: Arc<Mutex<BTreeSet<IpAddr>>>,
    nat: Arc<Mutex<BTreeMap<Identity, IpAddr>>>,
}

impl Endpoint {
    /// Create a new UDP endpoint handler
    pub fn new() -> Self {
        Self {
            ips: Default::default(),
            nat: Default::default(),
        }
    }

    /// Blocking call that runs
    pub fn run() {}
}
