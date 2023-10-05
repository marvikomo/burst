
#![allow(dead_code)]  //this turns of warning for vars I don't use
#![allow(unused_imports)]

//use super::transaction::Transaction;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use sha2::{Digest, Sha256};
use crate::blockchain::hash::CryptoHash;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub vesrion: Version,
    pub chain_id: String, //identifier od the blockchain
    pub height: u64, //the height of the block
    pub last_block_id: BlockID, //reference to the previous block, this ensures blocks are added in sequence and allows for entire chain to be verified from genesis to the latest block
    pub last_commit_hash:[u8; 32] , //Hash of the previous commit
    pub data_hash: [u8; 32], //Hash of the transaction in the block
    pub validators_hash:[u8; 32],  //Hash of the validator set for the current block
    pub next_validators_hash: [u8; 32],
    pub consensus_hash: [u8; 32], //consensus parameters might change overtime (e.g due to governance decisions). Storing a hash of these parameters ensures that all validators can participate in the consensus process
    pub app_hash: [u8; 32], //This allows tendermint to be agnostic to the application logic. It ensures that all validators' state machine process transactions consistently
    pub last_result_hash: [u8; 32],
    pub evidence_hash: [u8; 32], //To store evidence of any byzantine behaviour by validators. Storing evidence of malicious behavior ensures that malicious validators can be penalized
    pub proposer_address: [u8; 32], //This indicates which proposer proposed the block
    pub epoch_id: EpochId,
    pub next_epoch_id: EpochId,
    pub gas_price: u68,
    pub validator_reward: Balance,
    pub timestamp: u64,
    pub protocol_version: ProtocolVersion
}

struct Version {
    block: i64,
    app: i64
}

struct BlockID {
  hash: [u8; 32],
  parts_header: PartsHeader,
}

struct PartsHeader {
 total: i32,
 hash: [u8; 32]
}




#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    id: u64,
    block_header: BlockHeader,
    timestamp: i64,
    data: Data,
    evidence: Evidence,
    last_commit: LastCommit
}

struct Data {
    txs: Vec<Transaction>
}

struct Evidence {
    evidence: Vec<EnvidenceDetails>
}
//the structure of the transaction will depend on what BURST requires, we will look more into this but for now lets keep it simple but
//however, a transaction is often treated as a raw byte array since Tendermint is agnostic to the transaction's content
struct Transaction {

}

//Also lets look into what this will look like later
struct EnvidenceDetails {

}

impl Block {
    pub fn new(id:u64,transaction: Vec<Transaction>,previous_hash: String, validator: String) -> Self {
        //let hash = generate_hash(String::from("2"), timestamp: i64, hash: &str, previous_hash: &str)
        Self {
            id,
            timestamp: Utc::now().timestamp(),
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


