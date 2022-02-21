// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Table of all direct neighbour nodes
//! 
//! There is a table per connection module.

use libp2p::PeerId;
use state::Storage;
use prost::Message;
use std::{
    collections::HashMap,
    sync::RwLock,
    time::SystemTime,
};

use crate::connections::ConnectionModule;
use super::info::RouterInfo;
use super::proto;
use crate::rpc::Rpc;

/// mutable state of Neighbours table per ConnectionModule
static INTERNET: Storage<RwLock<Neighbours>> = Storage::new();
static LAN: Storage<RwLock<Neighbours>> = Storage::new();

pub struct Neighbours {
    nodes: HashMap<PeerId, Neighbour>,
}

pub struct Neighbour {
    /// round trip time in micro seconds
    rtt: u32,
    /// when was this node last seen
    updated_at: SystemTime,
}

impl Neighbours {
    pub fn init() {
        // neighbours table for internet connection module
        let internet = Neighbours { nodes: HashMap::new() };
        INTERNET.set(RwLock::new(internet));

        // neighbours table for lan connection module
        let lan = Neighbours { nodes: HashMap::new() };
        LAN.set(RwLock::new(lan));
    }

    /// update table with a new value
    /// 
    /// If the node already exists, it updates it's rtt value.
    /// If the node does not yet exist, it creates it.
    pub fn update_node( module: ConnectionModule, node_id: PeerId, rtt: u32 ) {
        // get table
        let mut neighbours;
        match module {
            ConnectionModule::Lan => neighbours = LAN.get().write().unwrap(),
            ConnectionModule::Internet => neighbours = INTERNET.get().write().unwrap(),
            ConnectionModule::Ble => return,
            ConnectionModule::Local => return,
            ConnectionModule::None => return,
        }

        // get node from table
        let node_option = neighbours.nodes.get_mut( &node_id );
        if let Some(node) = node_option {
            node.rtt = Self::calculate_rtt( node.rtt , rtt);
            node.updated_at = SystemTime::now();
        }
        else {
            log::info!("add node {:?} to neighbours table", node_id);
            neighbours.nodes.insert( node_id, Neighbour { rtt, updated_at: SystemTime::now() } );

            // add neighbour in RouterInfo neighbours table
            RouterInfo::add_neighbour(node_id);
        } 
    }

    /// Delete Neighbour
    #[allow(dead_code)]
    pub fn delete( module: ConnectionModule, node_id: PeerId ) {
        // get table
        let mut neighbours;
        match module {
            ConnectionModule::Lan => neighbours = LAN.get().write().unwrap(),
            ConnectionModule::Internet => neighbours = INTERNET.get().write().unwrap(),
            ConnectionModule::Ble => return,
            ConnectionModule::Local => return,
            ConnectionModule::None => return,
        }

        // delete entry
        neighbours.nodes.remove( &node_id );
    }

    /// Calculate average rtt
    fn calculate_rtt( old_rtt: u32, new_rtt: u32 ) -> u32 {
        (old_rtt * 3 + new_rtt) / 4
    }

    /// get rtt for a neighbour
    /// returns the round trip time for the neighbour in the 
    /// connection module.
    /// If the neighbour does not exist, it returns None.
    pub fn get_rtt( neighbour_id: &PeerId, module: &ConnectionModule ) -> Option<u32> {
        // get table
        let neighbours;
        match module {
            ConnectionModule::Lan => neighbours = LAN.get().read().unwrap(),
            ConnectionModule::Internet => neighbours = INTERNET.get().read().unwrap(),
            ConnectionModule::Ble => return None,
            ConnectionModule::Local => return Some(0),
            ConnectionModule::None => return None,
        }

        // search for neighbour
        if let Some(neighbour) = neighbours.nodes.get(neighbour_id) {
            return Some(neighbour.rtt)
        } else {
            return None
        }
    }

    /// Is this node ID a neighbour in any module?
    /// returns the first found module or `None`
    pub fn is_neighbour( node_id: &PeerId ) -> ConnectionModule {
        // check if neighbour is in Lan table
        {
            let lan = LAN.get().read().unwrap();
            if lan.nodes.contains_key(node_id) {
                return ConnectionModule::Lan
            }
        }
        // check if neighbour exists in Internet table
        {
            let internet = INTERNET.get().read().unwrap();
            if internet.nodes.contains_key(node_id) {
                return ConnectionModule::Internet
            }
        }

        ConnectionModule::None
    }

    /// send protobuf RPC neighbours list
    pub fn rpc_send_neighbours_list() {
        // create lists per module
        let mut lan_neighbours: Vec<proto::NeighboursEntry> = Vec::new();
        let mut internet_neighbours: Vec<proto::NeighboursEntry> = Vec::new();

        // fill lan connection module neighbours
        {
            println!("LAN neighbours:");
            let lan = LAN.get().read().unwrap();

            for (id, value) in &lan.nodes {
                lan_neighbours.push(proto::NeighboursEntry {
                    node_id: id.to_bytes(),
                    rtt: value.rtt,
                });
            }
        }
        
        // fill internet connection module neighbours
        {
            println!("Internet neighbours:");
            let internet = INTERNET.get().write().unwrap();

            for (id, value) in &internet.nodes {
                internet_neighbours.push(proto::NeighboursEntry {
                    node_id: id.to_bytes(),
                    rtt: value.rtt,
                });
            }
        }

        // create neighbours list message
        let proto_message = proto::Router {
            message: Some(proto::router::Message::NeighboursList(
                proto::NeighboursList {
                    lan: lan_neighbours,
                    internet: internet_neighbours,
                }
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, crate::rpc::proto::Modules::Router.into(), "".to_string(), Vec::new() );
    }
}

