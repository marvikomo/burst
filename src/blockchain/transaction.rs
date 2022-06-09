use std::fmt::Debug;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction{
    pub transaction_id: String,
    pub transaction_timestamp: i64,
    pub transaction_details: String
}

impl Transaction {
    pub fn new(transaction_id: String, transaction_timestamp: i64, transaction_details: String) -> Self{
        Self{
            transaction_id,
            transaction_timestamp,
            transaction_details
        }
    }
}