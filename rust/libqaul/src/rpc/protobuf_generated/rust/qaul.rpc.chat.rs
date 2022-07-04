/// Chat service RPC message container
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chat {
    /// message type
    #[prost(oneof="chat::Message", tags="1, 2, 3, 4, 5")]
    pub message: ::core::option::Option<chat::Message>,
}
/// Nested message and enum types in `Chat`.
pub mod chat {
    /// message type
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// request an overview over the last conversations
        #[prost(message, tag="1")]
        OverviewRequest(super::ChatOverviewRequest),
        /// contains the overview list
        #[prost(message, tag="2")]
        OverviewList(super::ChatOverviewList),
        /// request a specific conversation
        #[prost(message, tag="3")]
        ConversationRequest(super::ChatConversationRequest),
        /// list of a chat conversation
        #[prost(message, tag="4")]
        ConversationList(super::ChatConversationList),
        /// send a new chat message
        #[prost(message, tag="5")]
        Send(super::ChatMessageSend),
    }
}
/// request for overview list of all conversations
/// this request shall be sent continuously when the view is open
/// 
/// at the moment always the entire list is sent
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatOverviewRequest {
}
/// overview list of conversations
/// this can eighter be the entire list or the last updates
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatOverviewList {
    #[prost(message, repeated, tag="1")]
    pub overview_list: ::prost::alloc::vec::Vec<ChatOverview>,
}
/// a chat conversation overview item
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatOverview {
    /// id of the user
    #[prost(bytes="vec", tag="1")]
    pub conversation_id: ::prost::alloc::vec::Vec<u8>,
    /// last message index
    #[prost(uint32, tag="2")]
    pub last_message_index: u32,
    /// name of the conversation
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// time when last message was sent or received
    #[prost(uint64, tag="4")]
    pub last_message_at: u64,
    /// unread messages
    #[prost(int32, tag="5")]
    pub unread: i32,
    /// preview text of the last message
    #[prost(bytes="vec", tag="6")]
    pub content: ::prost::alloc::vec::Vec<u8>,
    /// sender of the last message
    #[prost(bytes="vec", tag="7")]
    pub last_message_sender_id: ::prost::alloc::vec::Vec<u8>,
}
/// request messages of a specific chat conversation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatConversationRequest {
    #[prost(bytes="vec", tag="1")]
    pub conversation_id: ::prost::alloc::vec::Vec<u8>,
    /// send only changes that are newer than the last received
    #[prost(uint64, tag="2")]
    pub last_index: u64,
}
/// list of chat messages of a specific conversation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatConversationList {
    #[prost(bytes="vec", tag="1")]
    pub conversation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="2")]
    pub message_list: ::prost::alloc::vec::Vec<ChatMessage>,
}
/// a single chat message
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessage {
    /// message index
    #[prost(uint32, tag="1")]
    pub index: u32,
    /// id of the sending user
    #[prost(bytes="vec", tag="2")]
    pub sender_id: ::prost::alloc::vec::Vec<u8>,
    /// message id
    #[prost(bytes="vec", tag="3")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
    /// message status
    /// 0 = nothing
    /// 1 = sent
    /// 2 = received
    #[prost(int32, tag="4")]
    pub status: i32,
    /// time when the message was sent
    #[prost(uint64, tag="5")]
    pub sent_at: u64,
    /// time when the message was received
    #[prost(uint64, tag="6")]
    pub received_at: u64,
    /// content of the message
    #[prost(bytes="vec", tag="7")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// chat message content
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessageContent {
    #[prost(oneof="chat_message_content::Content", tags="1, 2")]
    pub content: ::core::option::Option<chat_message_content::Content>,
}
/// Nested message and enum types in `ChatMessageContent`.
pub mod chat_message_content {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        /// chat content
        #[prost(message, tag="1")]
        ChatContent(super::ChatContent),
        /// file sharing content
        #[prost(message, tag="2")]
        FileContent(super::FileShareContent),
    }
}
///chat content 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatContent {
    ///content
    #[prost(string, tag="1")]
    pub content: ::prost::alloc::string::String,
}
///file sharing content
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileShareContent {
    ///file history index in DB
    #[prost(uint64, tag="1")]
    pub history_index: u64,
    ///file identifier
    #[prost(uint64, tag="2")]
    pub file_id: u64,
    ///file name
    #[prost(string, tag="3")]
    pub file_name: ::prost::alloc::string::String,
    ///file size
    #[prost(uint32, tag="4")]
    pub file_size: u32,
    ///file description
    #[prost(string, tag="5")]
    pub file_descr: ::prost::alloc::string::String,
}
/// send chat message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessageSend {
    /// conversation id to which this message is sent
    #[prost(bytes="vec", tag="1")]
    pub conversation_id: ::prost::alloc::vec::Vec<u8>,
    /// content of the message
    #[prost(string, tag="2")]
    pub content: ::prost::alloc::string::String,
}
