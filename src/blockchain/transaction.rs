use std::fmt::Debug;

#[derive(Debug)]
pub struct Transaction{
    transaction_id: String,
    transaction_timestamp: i64,
    transaction_details: String
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