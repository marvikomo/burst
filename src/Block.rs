use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    id: u64,
    timestamp: i64,
    hash: String,
    previous_hash: String,
    //transaction: vec [],
    Validator: String,
}

impl Block {
    pub fn new(previous_hash: ) -> Self {
        Block{}
    }
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

    pub fn get_block_count() -> u64{}



}
