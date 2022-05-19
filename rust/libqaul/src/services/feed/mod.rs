// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul Feed Service
//! 
//! The feed service sends and receives feed messages into the network.
//! Feed messages are not encrypted and for everybody to read.
//! They should reach everyone in the network.

//use bs58::decode;
use libp2p::{
    identity::{Keypair, PublicKey},
    PeerId,
};
use prost::Message;
use log::{info, error};
use serde::{Serialize, Deserialize};
use state::Storage;
use std::{sync::RwLock, convert::TryInto};
use std::collections::BTreeMap;
use sled_extensions::{
    DbExt,
    bincode::Tree,
};

use crate::node::{
    Node,
    user_accounts::{UserAccount, UserAccounts},
};

use crate::connections::{
    ConnectionModule,
    lan::Lan,
    internet::Internet,
};
use crate::router;
use crate::router::flooder::Flooder;
use crate::rpc::Rpc;
use crate::storage::database::DataBase;
use crate::utilities::timestamp;

/// Import protobuf message definition generated by 
/// the rust module prost-build.
pub mod proto { include!("qaul.rpc.feed.rs"); }
pub mod proto_net { include!("qaul.net.feed.rs"); }


/// mutable state of feed messages
static FEED: Storage<RwLock<Feed>> = Storage::new();

/// For storing in data base
#[derive(Serialize, Deserialize, Clone)]
pub struct FeedMessageData {
    // index of message in the data base
    pub index: u64,
    // hash of the message
    pub message_id: Vec<u8>,
    // user ID of the sender
    pub sender_id: Vec<u8>,
    // time sent in milli seconds
    pub timestamp_sent: u64,
    // time received in milli seconds
    pub timestamp_received: u64,
    // the message content
    pub content: String,
}
/// Feed message
#[derive(Debug, Clone)]
pub struct FeedMessage {
    /// the user id of the user sending this message
    pub sender: PeerId,
    /// the content of the message
    pub content: String,
    /// the time when this message was sent in seconds
    pub time: u64,
}

/// Serializable format of the feed message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedMessageSend {
    /// the user id of the user sending this message
    pub sender: Vec<u8>,
    /// the content of the message
    pub content: String,
    /// the time when this message was sent in seconds
    pub time: f64,
}

/// Feed message container
/// 
/// Contains the message and the message ID
/// which is verifiable signature of from the sending
/// user of the message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedMessageSendContainer {
    pub message: FeedMessageSend,
    pub id: Vec<u8>,
}

/// qaul Feed storage and logic
pub struct Feed {
    // in memory BTreeMap
    pub messages: BTreeMap< Vec<u8>, proto_net::FeedMessageContent>,
    // sled data base tree
    pub tree: Tree<FeedMessageData>,
    // last recent message
    pub last_message: u64,
}

impl Feed {
    /// initialize feed module
    pub fn init() {
        // get database and initialize tree
        let db = DataBase::get_node_db();
        let tree: Tree<FeedMessageData> = db.open_bincode_tree("feed").unwrap();

        // get last key
        let last_message: u64;
        match tree.iter().last() {
            Some(Ok((ivec, _))) => {
                let i = ivec.to_vec();
                match i.try_into() {
                    Ok(arr) => {
                        last_message = u64::from_be_bytes(arr);
                    }
                    Err(e) => {
                        log::error!("couldn't convert ivec to u64: {:?}", e);
                        last_message = 0;
                    }    
                }
            },
            None => {
                last_message = 0;
            },
            Some(Err(e)) => {
                log::error!("Sled feed table error: {}", e);
                last_message = 0;
            }
        }

        // create feed messages state
        let feed = Feed {
            messages: BTreeMap::new(),
            tree,
            last_message,
        };
        FEED.set(RwLock::new(feed));
    }

    /// Send message via all swarms
    pub fn send(user_account: &UserAccount, content: String,  lan: Option<&mut Lan>, internet: Option<&mut Internet> )
    {
        // create timestamp
        let timestamp = timestamp::Timestamp::get_timestamp();

        // create feed message
        let msg = proto_net::FeedMessageContent {
            sender: user_account.id.to_bytes(),
            content: content.clone(),
            time: timestamp,
        };

        // encode feed message
        let mut buf = Vec::with_capacity(msg.encoded_len());
        msg.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        // sign message
        let signature = Self::sign_message(&buf, user_account.keys.clone());

        // create signed container
        let container = proto_net::FeedContainer { signature , message: buf };
     
        // encode container
        let mut buf = Vec::with_capacity(container.encoded_len());
        container.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        // save message in feed store
        Self::save_message (container.signature.clone(), msg);

        // flood via floodsub
        if lan.is_some() {
            lan.unwrap().swarm.behaviour_mut().floodsub.publish(Node::get_topic(), buf.clone());
        }
        if internet.is_some() {
            internet.unwrap().swarm.behaviour_mut().floodsub.publish(Node::get_topic(), buf);
        }
    }

    /// Process a received message
    pub fn received( via_conn: ConnectionModule, _via_node: PeerId, feed_container: proto_net::FeedContainer ) {
                   
                match proto_net::FeedMessageContent::decode(&feed_container.message[..]) {
                    Ok(feed_content) => { 

                        let message = feed_content.clone();
                        
                        if let Ok(user_id_decoded) = PeerId::from_bytes(&message.sender){

                          // check if sending user public is in user store
                          let result = router::users::Users::get_pub_key(&user_id_decoded);

                          if let Some(key) = result {
                            // validate message
                            if !Self::validate_message(&feed_container, key.clone()) {
                                error!("Validation of feed message {:?} failed: {:?}", feed_container.signature, message.content);
                                error!("  sender id:  {}", user_id_decoded);
                                let (key_type, key_base58) = crate::router::users::Users::get_protobuf_public_key(key);
                                error!("  sender key [{}]: {}", key_type, key_base58);
                                return
                            }
                            
                            // check if message exists is in feed store
                            let mut new_message = true;

                            {
                                let feed = FEED.get().read().unwrap();

                                if feed.messages.contains_key(&feed_container.signature) {
                                    new_message = false;
                                }
                            }

                            // check if message exists
                            if new_message {
                                // write message to store
                                Self::save_message(feed_container.signature.clone(), feed_content);

                                // display message
                                info!("message received:");
                                info!("Timestamp - {}, Signature- {:?}", message.time, feed_container.signature);
                                info!(" Message Content {}", message.content);
                                
                                // encode container
                                let mut buf = Vec::with_capacity(feed_container.encoded_len());
                                feed_container.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

                                // forward message
                                Flooder::add(buf, Node::get_topic(), via_conn);
                            } else {
                                info!("message key {:?} already in store", feed_container.signature);
                            }

                            } else {
                                error!("Sender of feed message not known: {}", user_id_decoded);
                                return
                            }
                        }
                    }
                    Err(error) => {
                        log::error!("{:?}", error);
                    },
                }  
    }

    //Save message by sync
    pub fn save_message_by_sync(message_id: &Vec<u8>, sender_id: &Vec<u8>, content: String, time: u64) {

        let feedr = FEED.get().read().unwrap();
        if feedr.messages.contains_key(message_id) == true {
            return;
        }

        let msg_content = proto_net::FeedMessageContent {
            sender: sender_id.clone(),
            content: content.clone(),
            time: time
        };

        // open feed map for writing
        let mut feed = FEED.get().write().unwrap();

        // insert message to in memory BTreeMap
        feed.messages.insert(message_id.clone(), msg_content);

        // create new key
        let last_message = feed.last_message +1;

        // create timestamp
        let timestamp_received = timestamp::Timestamp::get_timestamp();
         
        //create feed struct for database store
        let message_data = FeedMessageData {
            index: last_message,
            message_id: message_id.clone(),
            sender_id: sender_id.clone(),
            timestamp_sent: time,
            timestamp_received,
            content: content.clone(),
        };    

        // save to data base
        if let Err(e) = feed.tree.insert(&last_message.to_be_bytes(), message_data) {
            log::error!("Error saving feed message to data base: {}", e);
        }
        else {
            if let Err(e) = feed.tree.flush() {
                log::error!("Error when flushing data base to disk: {}", e);
            }
        }

        // update key
        feed.last_message = last_message;
    }


    /// Save a Message
    /// 
    /// This function saves a new message in the data base and in the in-memory BTreeMap
    fn save_message(signature: Vec<u8>, message: proto_net::FeedMessageContent) {
        // open feed map for writing
        let mut feed = FEED.get().write().unwrap();

        // insert message to in memory BTreeMap
        feed.messages.insert(signature.clone(), message.clone());

        // create new key
        let last_message = feed.last_message +1;

        // create timestamp
        let timestamp_received = timestamp::Timestamp::get_timestamp();
         
        //create feed struct for database store
        let message_data = FeedMessageData {
            index: last_message,
            message_id: signature.clone(),
            sender_id: message.sender,
            timestamp_sent: message.time,
            timestamp_received,
            content: message.content.clone(),
        };    

        // save to data base
        if let Err(e) = feed.tree.insert(&last_message.to_be_bytes(), message_data) {
            log::error!("Error saving feed message to data base: {}", e);
        }
        else {
            if let Err(e) = feed.tree.flush() {
                log::error!("Error when flushing data base to disk: {}", e);
            }
        }

        // update key
        feed.last_message = last_message;
    }


    pub fn get_latest_message_ids(count: usize) -> Vec<Vec<u8>> {
        let mut ids: Vec<Vec<u8>> = vec![];

        // get feed message store
        let feed = FEED.get().read().unwrap();
        let mut msg_count: usize = count;
        if feed.last_message < (count as u64) {
            msg_count = feed.last_message as usize;
        }

        let first_message = feed.last_message - (msg_count as u64);
        let first_message_bytes = first_message.to_be_bytes().to_vec();
        for res in feed.tree.range(first_message_bytes.as_slice()..) {
            match res {
                Ok((_id, message)) => {
                    ids.push(message.message_id.clone());
                },
                Err(e) => {
                    log::error!("Error retrieving feed message from data base: {}", e);
                }
            }
        }
        ids
    }


    //return missing feed ids to request to the neighbour
    pub fn process_received_feed_ids(ids: &Vec<Vec<u8>>) -> Vec<Vec<u8>>{
        let mut missing_ids: Vec<Vec<u8>> = vec![];

        let feed = FEED.get().read().unwrap();
        for id in ids{
            if feed.messages.contains_key(id) == false {
                missing_ids.push(id.clone());
            }    
        }
        missing_ids
    }

    pub fn get_messges_by_ids(ids: &Vec<Vec<u8>>) -> Vec<(Vec<u8>, Vec<u8>, String, u64)>{
        let mut res: Vec<(Vec<u8>, Vec<u8>, String, u64)> = vec![];
        let feed = FEED.get().read().unwrap();
        for id in ids{
            if let Some(feed) = feed.messages.get(id) {
                res.push((id.clone(), feed.sender.clone(), feed.content.clone(), feed.time));
            }
        }
        res
    }


    /// Get messages from data base
    /// 
    /// This function get messages from data base
    /// that are newer then the last message.
    fn get_messages(last_message: u64) -> proto::FeedMessageList {
        // create empty feed list
        let mut feed_list = proto::FeedMessageList {
            feed_message: Vec::new(),
        };

        // get feed message store
        let feed = FEED.get().read().unwrap();

        // check if there are any new messages
        if feed.last_message > last_message {
            let first_message = last_message +1;
            let first_message_bytes = first_message.to_be_bytes().to_vec();
            // get all messages that are newer 
            // and there fore have a higher key.
            for res in feed.tree.range(first_message_bytes.as_slice()..) {
                match res {
                    Ok((_id, message)) => {
                        let sender_id_base58 = bs58::encode(message.sender_id.clone()).into_string();

                        //create timestamp
                        let time_sent = timestamp::Timestamp::create_time();
                        
                        // create message
                        let feed_message = proto::FeedMessage {
                            sender_id: message.sender_id.clone(),
                            // DEPRECATED
                            sender_id_base58,
                            message_id: message.message_id.clone(),
                            // DEPRECATED
                            message_id_base58: bs58::encode(message.message_id).into_string(),
                            // DEPRECATED
                            time_sent: humantime::format_rfc3339(time_sent.clone()).to_string(),
                            timestamp_sent: message.timestamp_sent,
                            // DEPRECATED
                            time_received: humantime::format_rfc3339(time_sent).to_string(),
                            timestamp_received: message.timestamp_received,
                            content: message.content.clone(),
                            // data base index
                            index: message.index,
                        };

                        // add message to feed list
                        feed_list.feed_message.push(feed_message);
                    },
                    Err(e) => {
                        log::error!("Error retrieving feed message from data base: {}", e);
                    }
                }
            }
        }

        feed_list
    }

    /// Sign a message with the private key
    /// The signature can be validated with the corresponding public key.
    pub fn sign_message ( buf: &Vec<u8> , keys: Keypair ) -> Vec<u8> {
        keys.sign(&buf).unwrap()
    }

    /// validate a message via the public key of the sender
    pub fn validate_message( msg: &proto_net::FeedContainer, key: PublicKey ) -> bool {
        key.verify(&msg.message, &msg.signature)
    }

    /// Process incoming RPC request messages for feed module
    pub fn rpc(data: Vec<u8>, user_id: Vec<u8>, lan: Option<&mut Lan>, internet: Option<&mut Internet> ) {
        match proto::Feed::decode(&data[..]) {
            Ok(feed) => {
                match feed.message {
                    Some(proto::feed::Message::Request(feed_request)) => {
                        // get feed messages from data base
                        let feed_list = Self::get_messages(feed_request.last_index);

                        // pack message
                        let proto_message = proto::Feed {
                            message: Some( 
                                proto::feed::Message::Received(feed_list)
                            ),
                        };

                        // encode message
                        let mut buf = Vec::with_capacity(proto_message.encoded_len());
                        proto_message.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

                        // send message
                        Rpc::send_message(buf, crate::rpc::proto::Modules::Feed.into(), "".to_string(), Vec::new() );
                    },
                    Some(proto::feed::Message::Send(send_feed)) => {
                        // print message
                        log::info!("feed message received: {}", send_feed.content.clone());

                        // get user account from user_id
                        let user_account;
                        match PeerId::from_bytes(&user_id){
                            Ok(user_id_decoded) => {
                                match UserAccounts::get_by_id(user_id_decoded) {
                                    Some(account) => {
                                        user_account = account;
                                        // send the message
                                        Self::send( &user_account, send_feed.content, lan, internet );
                                    },
                                    None => {
                                        log::error!("user account id not found: {:?}", user_id_decoded.to_base58());
                                        return
                                    },
                                }    
                            },
                            Err(e) => {
                                log::error!("user account id could'nt be encoded: {:?}", e);
                            },
                        }
                    },
                    _ => {
                        log::error!("Unhandled Protobuf Feed Message");
                    },
                }    
            },
            Err(error) => {
                log::error!("{:?}", error);
            },
        }
    }
}
