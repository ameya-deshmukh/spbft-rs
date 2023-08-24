use std::time::{SystemTime, UNIX_EPOCH};
pub struct client{
  client_to_primary : Message::Request(request{timesamp:SystemTime::now(),client:primary});
}
pub struct primary{
}
pub struct replica{
  replica_to_client : 
}
