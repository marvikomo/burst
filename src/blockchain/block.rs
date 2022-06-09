use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use super::transaction::Transaction;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};



#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    id: u64,
    timestamp: i64,
    hash: String,
    previous_hash: String,
    //transaction: vec [],
    validator: String,
    block_transactions: Vec<Transaction>
}

impl Block {
    pub fn new(id: u64, previous_hash: String, validator: String) -> Self {
       Self{
           id,
           timestamp: Utc::now().timestamp(),
           hash: String::from("00x"),
           previous_hash: String::from("00x"),
           validator: String::from("00x"),
           block_transactions: String::from("00x")
       }
    }
    pub fn genesis() -> Self {
        Block {
            id: 1,
            timestamp: Utc::now().timestamp(),
            hash: String::from("0"),
            previous_hash: String::from("0"),
            validator: String::from("0"),
            block_transactions: String::from("00x")
        }
    }
    
    pub fn generate_hash(id: u64, timestamp: i64, hash: &str, previous_hash: &str) -> String {
        unimplemented!();
    }

    //pub fn generate_block(&self, )

    pub fn is_valid_block(hash: &str) -> bool {
        unimplemented!();
    }

    pub fn serialize_block(&self) -> String {
        serde_json::to_string(&self).unwrap();
        unimplemented!();
    }

    pub fn get_block_count() -> u64 {
        unimplemented!();
    }
}

// impl Debug for Block{
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         write!(f, "{}", "error")
//     }
// }

