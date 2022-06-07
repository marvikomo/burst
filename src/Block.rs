use chrono::prelude::*;
use serde_json::{Result, Value};
pub struct Block {
    id: u64,
    timestamp: i64,
    hash: String,
    previous_hash: String,
    //transaction: vec [],
    Validator: String,
}

impl Block {
    pub fn genesis() -> Self {
        Block {
            id: 1,
            timestamp: Utc::now().timestamp(),
            hash: String::from("0"),
            previous_hash: String::from("0"),
            Validator: String::from("0"),
        }
    }

    pub fn is_valid_block(hash: &str) -> bool {

    }

    pub fn serialize_block(&self) -> String{
        serde_json::to_string(&self).unwrap();
    }

    pub fn generate_hash(block: &Block) -> String {

    }

}
