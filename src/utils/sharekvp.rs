use typemap::Key;
use std::collections::HashMap;
use chrono::{DateTime, Local};

pub struct CommandCounter;

impl Key for CommandCounter {
    type Value = HashMap<String, u64>;
}

pub struct StartupTime;

impl Key for StartupTime {
    type Value = DateTime<Local>;
}

pub struct CleverbotToken;

impl Key for CleverbotToken {
    type Value = String;
}