use std::net::SocketAddr;

use serde::{Serialize, Deserialize, Serializer};

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
   Request(Request),
   //PrePrepare(Prepare),
   //Prepare(Prepare),
   //Commit(Commit)
}

//Client sends a message <Request, o, t, c> to the primary
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Request{
    operation: String,
    timestamp: u64,
    client: SocketAddr, 
}

