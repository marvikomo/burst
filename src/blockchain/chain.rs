
#![allow(dead_code)]  //this turns of warning for vars I don't use
#![allow(unused_imports)]

use super::transaction::Transaction;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use sha2::{Digest, Sha256};

//#[derive(Serialize, Deserialize, Debug)]
// pub struct BlockHeader {
//     pub height: u64,
//     pub hash: CryptoHash,
//     pub previous_hash: CryptoHash,
//     pub epoch_id: EpochId,
//     pub next_epoch_id: EpochId,
//     pub gas_price: Balance,
//     pub validator_reward: Balance,
//     pub timestamp: u64,
//     pub signature: Signature,
//     pub protocol_version: ProtocolVersion
// }


#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    id: u64,
    timestamp: i64,
    hash: Option<String>,
    previous_hash: Option<String>,
    validator: Option<String>,
    block_transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(id:u64,transaction: Vec<Transaction>,previous_hash: String, validator: String) -> Self {
        //let hash = generate_hash(String::from("2"), timestamp: i64, hash: &str, previous_hash: &str)
        Self {
            id,
            timestamp: Utc::now().timestamp(),
            hash: None,
            previous_hash: Some(String::from("00x")),
            validator: Some(String::from("00x")),
            block_transactions:vec![]
        }
    }
    pub fn genesis() -> Self {
        let transaction: Transaction = Transaction::new(
            String::from("1"),
            Utc::now().timestamp(),
            String::from("This is a dummy transaction signed by marv"),
        );
        Block {
            id: 1,
            timestamp: Utc::now().timestamp(),
            hash: Some(String::from("0")),
            previous_hash: Some(String::from("0")),
            validator: Some(String::from("0")),
            block_transactions: vec![transaction]
        }
    }
    pub fn generate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.serialize_block().to_string().as_bytes());
        byte_vector_to_string(&hasher.finalize().as_slice().to_owned())    
    }

    //pub fn generate_block(&self, )

    pub fn is_valid_block(hash: &str) -> bool {
        unimplemented!();
    }

    pub fn serialize_block(&self) -> String {
        serde_json::to_string(&self).unwrap()   
    }

    pub fn get_block_count() -> u64 {
        unimplemented!();
    }
}

fn byte_vector_to_string(arr: &Vec<u8>) -> String {
    arr.iter().map(|&c| c as char).collect()
}


