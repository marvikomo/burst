use sha2::Digest;
use borsh::{BorshSerialize, BorshDeserialize};

pub struct CryptoHash(pub [u8; 32]);

impl CryptoHash {

    pub const fn new() -> Self {
        Self([0; Self::LENGTH])
    }

    pub fn calculate_hash(bytes: &[u8]) -> CryptoHash {
        CryptoHash(sha2::Sha256::digest(bytes).into())
     }



}



