use futures::channel::mpsc::UnboundedSender;
use playhub_proto::StreamId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type ActiveStreams = Arc<RwLock<HashMap<StreamId, UnboundedSender<StreamMessage>>>>;

#[derive(Debug, Clone)]
pub enum StreamMessage {
    Data(Vec<u8>),
    Close,
}
pub mod error;
pub mod local;
pub mod proxy_client;
