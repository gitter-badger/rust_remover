use typemap::Key;
use std::collections::HashMap;
use time::Tm;

pub struct CommandCounter;

impl Key for CommandCounter {
    type Value = HashMap<String, u64>;
}

pub struct StartupTime;

impl Key for StartupTime {
    type Value = Tm;
}

#[derive(Debug)]
pub struct ReducedReadyPayload {
    pub session_id: String,
    pub shard: Option<[u64; 2]>,
    pub version: u64
}

impl Key for ReducedReadyPayload {
    type Value = ReducedReadyPayload;
}
